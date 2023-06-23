use std::{collections::HashMap, sync::Arc, time::Duration};

use chrono::LocalResult;
use reqwest::{Client, ClientBuilder};
use serde::Deserialize;
use tracing::{info, instrument};
use url::Url;

use crate::{models::vehicle::Vehicle, prelude::*};

/// Wargaming.net API client.
#[derive(Clone)]
pub struct Wg {
    client: Client,
    application_id: Arc<String>,
}

#[derive(Deserialize)]
pub struct WgError {
    pub code: u16,
    pub message: String,
}

impl From<WgError> for Error {
    fn from(error: WgError) -> Self {
        anyhow!("Wargaming.net API error #{} ({})", error.code, error.message)
    }
}

/// Wargaming.net API result.
#[derive(Deserialize)]
#[serde(tag = "status")]
pub enum WgResult<D> {
    #[serde(rename = "ok")]
    Ok { data: D },

    #[serde(rename = "error")]
    Err { error: WgError },
}

impl Wg {
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
    #[cfg(not(test))]
    #[instrument(skip_all, fields(account_id = account_id))]
    pub async fn get_vehicles_stats(&self, account_id: u32) -> Result<Vec<VehicleStats>> {
        let result = self
            .client
            .get(Url::parse_with_params(
                "https://api.wotblitz.eu/wotb/tanks/stats/",
                &[
                    ("application_id", self.application_id.as_str()),
                    ("account_id", account_id.to_string().as_str()),
                    ("fields", "tank_id,last_battle_time,all.battles"),
                ],
            )?)
            .send()
            .await
            .with_context(|| format!("failed to retrieve player {account_id}'s vehicles stats"))?
            .json::<WgResult<HashMap<String, Vec<VehicleStats>>>>()
            .await
            .with_context(|| format!("failed to parse player {account_id}'s vehicles stats"))?;
        match result {
            WgResult::Ok { data } => Ok(data.into_values().next().unwrap_or_default()),
            WgResult::Err { error } => Err(error.into()),
        }
    }

    #[cfg(test)]
    pub async fn get_vehicles_stats(&self, _account_id: u32) -> Result<Vec<VehicleStats>> {
        const FAKE_NON_PLAYED: VehicleStats = VehicleStats {
            tank_id: 2,
            last_battle_time: 0,
            inner: InnerVehicleStats { n_battles: 0 },
        };
        const FAKE_PLAYED: VehicleStats = VehicleStats {
            tank_id: 1,
            last_battle_time: 0,
            inner: InnerVehicleStats { n_battles: 1 },
        };
        Ok(vec![FAKE_PLAYED, FAKE_NON_PLAYED])
    }

    /// Retrieve the [tankopedia][1].
    ///
    /// [1]: https://developers.wargaming.net/reference/all/wotb/encyclopedia/vehicles/
    #[instrument(skip_all)]
    pub async fn get_tankopedia(&self) -> Result<Vec<Vehicle>> {
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
            .json::<WgResult<HashMap<String, Vehicle>>>()
            .await
            .context("failed to parse the tankopedia")?;
        match result {
            WgResult::Ok { data } => Ok(data.into_values().collect()),
            WgResult::Err { error } => Err(error.into()),
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
            .json::<WgResult<()>>()
            .await
            .context("failed to parse the log-out response")?;
        match result {
            WgResult::Ok { .. } => Ok(()),
            WgResult::Err { error } => Err(error.into()),
        }
    }
}

/// Partial user's vehicle statistics.
#[derive(Deserialize)]
pub struct VehicleStats {
    pub tank_id: u16,
    pub last_battle_time: i64,

    #[serde(rename = "all")]
    pub inner: InnerVehicleStats,
}

impl VehicleStats {
    pub fn last_battle_time(&self) -> LocalResult<DateTime> {
        Utc.timestamp_opt(self.last_battle_time, 0)
    }

    pub const fn is_played(&self) -> bool {
        self.inner.n_battles != 0
    }
}

#[derive(Deserialize)]
pub struct InnerVehicleStats {
    #[serde(rename = "battles")]
    pub n_battles: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vehicles_stats_ok() -> Result {
        serde_json::from_str::<WgResult<HashMap<String, Vec<VehicleStats>>>>(
            // language=json
            r#"{"status":"ok","meta":{"count":1},"data":{"594778041":[{"all":{"battles":248},"last_battle_time":1681146251,"tank_id":18769}]}}"#,
        )?;
        Ok(())
    }
}
