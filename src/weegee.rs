use std::{collections::HashMap, sync::Arc, time::Duration};

use anyhow::Context;
use moka::future::Cache;
use reqwest::{Client, ClientBuilder, Url};
use serde::Deserialize;
use tracing::instrument;

use crate::prelude::*;

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

impl<D> From<WeeGeeResult<D>> for Result<D> {
    fn from(value: WeeGeeResult<D>) -> Self {
        match value {
            WeeGeeResult::Ok { data } => Ok(data),
            WeeGeeResult::Err { error } => {
                Err(anyhow!("Wargaming.net API error #{}: {}", error.code, error.message))
            }
        }
    }
}

impl WeeGee {
    pub fn new(application_id: &str) -> Result<Self> {
        let client = ClientBuilder::new()
            .gzip(true)
            .tcp_nodelay(true)
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(10))
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
    pub async fn get_vehicles_stats(&self, account_id: u32) -> Result<VehiclesStats> {
        self.client
            .get(Url::parse_with_params(
                "https://api.wotblitz.eu/wotb/tanks/stats/",
                &[
                    ("application_id", self.application_id.as_str()),
                    ("account_id", account_id.to_string().as_str()),
                    ("fields", "tank_id,last_battle_time"),
                ],
            )?)
            .send()
            .await?
            .json::<WeeGeeResult<VehiclesStats>>()
            .await?
            .into()
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

/// Convenience alias for the map returned by the API.
pub type VehiclesStats = HashMap<String, Vec<VehicleStats>>;

/// Proxy for user's vehicles' statistics.
#[derive(Clone)]
pub struct VehicleStatsGetter {
    wee_gee: WeeGee,
    cache: Cache<u32, Arc<VehiclesStats>>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vehicles_stats_ok() -> Result {
        serde_json::from_str::<WeeGeeResult<VehiclesStats>>(
            // language=json
            r#"{"status":"ok","meta":{"count":1},"data":{"594778041":[{"last_battle_time":1681146251,"tank_id":18769}]}}"#,
        )?;
        Ok(())
    }
}
