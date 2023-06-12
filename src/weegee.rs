use std::{collections::HashMap, sync::Arc, time::Duration};

use anyhow::Context;
use chrono::LocalResult;
use indexmap::IndexMap;
use itertools::Itertools;
use moka::future::Cache;
use reqwest::{Client, ClientBuilder, Url};
use serde::Deserialize;
use tracing::{info, instrument};

use crate::{models::VehicleDescription, prelude::*};

/// Wargaming.net API client.
#[derive(Clone)]
pub struct WeeGee {
    client: Client,
    application_id: Arc<String>,
}

/// Wargaming.net API result.
#[derive(Deserialize)]
#[serde(tag = "status")]
enum WeeGeeResult<D> {
    #[serde(rename = "ok")]
    Ok { data: D },

    #[serde(rename = "error")]
    Err { error: WeeGeeError },
}

#[derive(Deserialize)]
struct WeeGeeError {
    pub code: u16,
    pub message: String,
}

impl From<WeeGeeError> for Error {
    fn from(error: WeeGeeError) -> Self {
        anyhow!("Wargaming.net API error #{} ({})", error.code, error.message)
    }
}

impl WeeGee {
    pub fn new(application_id: &str) -> Result<Self> {
        let client = ClientBuilder::new()
            .gzip(true)
            .tcp_nodelay(true)
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(10))
            .use_rustls_tls()
            .build()
            .context("failed to build the Wargaming.net API client")?;
        Ok(Self {
            client,
            application_id: Arc::new(application_id.to_string()),
        })
    }

    /// Get the accounts' vehicles' basic [statistics][1].
    ///
    /// [1]: https://developers.wargaming.net/reference/all/wotb/tanks/stats/
    #[instrument(skip_all, fields(account_id = account_id))]
    pub async fn get_vehicles_stats(&self, account_id: u32) -> Result<Vec<VehicleStats>> {
        if account_id == 0 {
            // Fake account ID for testing.
            return Ok(Vec::default());
        }
        let result = self
            .client
            .get(Url::parse_with_params(
                "https://api.wotblitz.eu/wotb/tanks/stats/",
                &[
                    ("application_id", self.application_id.as_str()),
                    ("account_id", account_id.to_string().as_str()),
                    ("fields", "tank_id,last_battle_time"),
                ],
            )?)
            .send()
            .await
            .with_context(|| format!("failed to retrieve player {account_id}'s vehicles stats"))?
            .json::<WeeGeeResult<HashMap<String, Vec<VehicleStats>>>>()
            .await
            .with_context(|| format!("failed to parse player {account_id}'s vehicles stats"))?;
        match result {
            WeeGeeResult::Ok { data } => Ok(data.into_values().next().unwrap_or_default()),
            WeeGeeResult::Err { error } => Err(error.into()),
        }
    }

    /// Retrieve the [tankopedia][1].
    ///
    /// [1]: https://developers.wargaming.net/reference/all/wotb/encyclopedia/vehicles/
    #[instrument(skip_all)]
    pub async fn get_tankopedia(&self) -> Result<Vec<VehicleDescription>> {
        info!("☎️ Retrieving the tankopedia…");
        let result = self
            .client
            .get(Url::parse_with_params(
                "https://api.wotblitz.eu/wotb/encyclopedia/vehicles/",
                &[
                    ("application_id", self.application_id.as_str()),
                    ("language", "en"),
                    ("fields", "tank_id,name,images.normal,is_premium"),
                ],
            )?)
            .send()
            .await
            .context("failed to retrieve the tankopedia")?
            .json::<WeeGeeResult<HashMap<String, VehicleDescription>>>()
            .await
            .context("failed to parse the tankopedia")?;
        match result {
            WeeGeeResult::Ok { data } => Ok(data.into_values().collect()),
            WeeGeeResult::Err { error } => Err(error.into()),
        }
    }

    /// [Log out][1].
    ///
    /// [1]: https://developers.wargaming.net/reference/all/wot/auth/logout/
    #[instrument(skip_all)]
    pub async fn log_out(&self, access_token: &str) -> Result {
        let result = self
            .client
            .post("https://api.worldoftanks.eu/wot/auth/logout/")
            .form(&[
                ("application_id", self.application_id.as_str()),
                ("access_token", access_token),
            ])
            .send()
            .await
            .context("failed to log out")?
            .json::<WeeGeeResult<()>>()
            .await
            .context("failed to parse the log-out response")?;
        match result {
            WeeGeeResult::Ok { .. } => Ok(()),
            WeeGeeResult::Err { error } => Err(error.into()),
        }
    }
}

/// User's vehicle statistics.
///
/// We only need tank ID and last battle time for the app's purposes.
#[derive(Deserialize)]
pub struct VehicleStats {
    pub tank_id: u16,
    pub last_battle_time: i64,
}

impl VehicleStats {
    pub fn last_battle_time(&self) -> LocalResult<DateTime> {
        Utc.timestamp_opt(self.last_battle_time, 0)
    }
}

/// Proxy for user's vehicles' statistics.
#[derive(Clone)]
pub struct VehicleStatsGetter {
    wee_gee: WeeGee,
    cache: Cache<u32, Arc<IndexMap<u16, VehicleStats>>>,
}

impl From<WeeGee> for VehicleStatsGetter {
    fn from(wee_gee: WeeGee) -> Self {
        Self {
            wee_gee,
            cache: Cache::builder()
                .max_capacity(1000)
                .time_to_idle(Duration::from_secs(300))
                .build(),
        }
    }
}

impl VehicleStatsGetter {
    /// Retrieve the account's vehicle's statistics and cache it.
    #[instrument(skip_all, fields(account_id = account_id))]
    pub async fn get(&self, account_id: u32) -> Result<Arc<IndexMap<u16, VehicleStats>>> {
        self.cache
            .try_get_with(account_id, async {
                let map = self
                    .wee_gee
                    .get_vehicles_stats(account_id)
                    .await?
                    .into_iter()
                    .sorted_unstable_by_key(|stats| -stats.last_battle_time)
                    .map(|stats| (stats.tank_id, stats))
                    .collect();
                Ok(Arc::new(map))
            })
            .await
            .map_err(|error: Arc<Error>| anyhow!(error))
            .with_context(|| format!("failed to retrieve account {account_id}'s vehicles stats"))
    }

    #[instrument(skip_all, fields(account_id = account_id, tank_id = tank_id))]
    pub async fn owns_vehicle(&self, account_id: u32, tank_id: u16) -> Result<bool> {
        Ok(self.get(account_id).await?.contains_key(&tank_id))
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Nation {
    Ussr,
    Germany,
    Usa,
    China,
    France,
    Uk,
    Japan,
    Europe,
    Other,
}

impl TryFrom<u16> for Nation {
    type Error = Error;

    fn try_from(tank_id: u16) -> Result<Self> {
        const COMPONENT_VEHICLE: u16 = 1;
        debug_assert_eq!(tank_id & COMPONENT_VEHICLE, COMPONENT_VEHICLE);

        const NATIONS: [Nation; 9] = [
            Nation::Ussr,
            Nation::Germany,
            Nation::Usa,
            Nation::China,
            Nation::France,
            Nation::Uk,
            Nation::Japan,
            Nation::Other,
            Nation::Europe,
        ];

        let index = ((tank_id >> 4) & 0xF) as usize;
        NATIONS
            .get(index)
            .copied()
            .ok_or_else(|| anyhow!("unexpected nation {index} for tank {tank_id}"))
    }
}

impl Nation {
    pub const fn get_id(self) -> u32 {
        match self {
            Nation::Ussr => 20000,
            Nation::Germany => 30000,
            Nation::Usa => 10000,
            Nation::China => 60000,
            Nation::France => 70000,
            Nation::Uk => 40000,
            Nation::Japan => 50000,
            Nation::Other => 100000,
            Nation::Europe => 80000,
        }
    }
}

pub fn to_client_tank_id(api_tank_id: u16) -> Result<u32> {
    Ok(Nation::try_from(api_tank_id)?.get_id() + (api_tank_id as u32 >> 8))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vehicles_stats_ok() -> Result {
        serde_json::from_str::<WeeGeeResult<HashMap<String, Vec<VehicleStats>>>>(
            // language=json
            r#"{"status":"ok","meta":{"count":1},"data":{"594778041":[{"last_battle_time":1681146251,"tank_id":18769}]}}"#,
        )?;
        Ok(())
    }

    #[test]
    fn to_client_tank_id_ok() -> Result {
        assert_eq!(to_client_tank_id(2817)?, 20011); // USSR
        assert_eq!(to_client_tank_id(54289)?, 30212); // Germany
        assert_eq!(to_client_tank_id(52257)?, 10204); // USA
        assert_eq!(to_client_tank_id(9009)?, 60035); // China
        assert_eq!(to_client_tank_id(18257)?, 40071); // UK
        assert_eq!(to_client_tank_id(5953)?, 70023); // France
        assert_eq!(to_client_tank_id(4193)?, 50016); // Japan
        assert_eq!(to_client_tank_id(5489)?, 100021); // Other
        assert_eq!(to_client_tank_id(1409)?, 80005); // Europe
        Ok(())
    }
}
