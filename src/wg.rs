use std::{collections::HashMap, sync::Arc, time::Duration};

use reqwest::{Client, ClientBuilder};
use serde::{Deserialize, Deserializer};
use tracing::instrument;

use crate::{models::AccountId, prelude::*};

/// Wargaming.net API client.
#[derive(Clone)]
pub struct Wg {
    client: Client,
    application_id: Arc<String>,
}

/// Generic Wargaming.net API response.
#[derive(Deserialize)]
#[serde(tag = "status")]
pub enum WgResponse<D> {
    #[serde(rename = "ok")]
    Ok { data: D },

    #[serde(rename = "error")]
    Err { error: WgError },
}

#[derive(Deserialize, Debug, thiserror::Error)]
#[serde(untagged)]
pub enum WgError {
    #[error("invalid token")]
    InvalidToken {
        code: monostate::MustBe!(407),
        message: monostate::MustBe!("INVALID_ACCESS_TOKEN"),
    },

    #[error("Wargaming.net API error `{code}/{message}`")]
    Api { code: u16, message: String },

    #[serde(skip_deserializing)]
    #[error("request error: {0:#}")]
    Request(#[from] Error),
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

    #[cfg(not(test))]
    #[instrument(skip_all, fields(account_id = account_id))]
    pub async fn get_account_info(
        &self,
        account_id: AccountId,
        access_token: &str,
    ) -> Result<Option<AccountInfo>, WgError> {
        let url = url::Url::parse_with_params(
            "https://api.wotblitz.eu/wotb/account/info/",
            &[
                ("application_id", self.application_id.as_str()),
                ("account_id", account_id.to_string().as_str()),
                ("access_token", access_token),
                ("fields", "private"),
            ],
        )
        .context("failed to construct the URL")?;
        let result = self
            .client
            .get(url)
            .send()
            .await
            .with_context(|| format!("failed to retrieve player {account_id}'s info"))?
            .json::<WgResponse<HashMap<String, Option<AccountInfo>>>>()
            .await
            .with_context(|| format!("failed to parse player {account_id}'s info"))?;
        match result {
            WgResponse::Ok { data } => Ok(data.into_values().next().unwrap_or_default()),
            WgResponse::Err { error } => Err(error),
        }
    }

    #[cfg(test)]
    pub async fn get_account_info(
        &self,
        _account_id: AccountId,
        _access_token: &str,
    ) -> Result<Option<AccountInfo>, WgError> {
        Ok(Some(AccountInfo {
            private: serde_json::Value::Object(Default::default()),
        }))
    }

    /// Get the accounts' vehicles' basic [statistics][1].
    ///
    /// [1]: https://developers.wargaming.net/reference/all/wotb/tanks/stats/
    #[cfg(not(test))]
    #[instrument(skip_all, fields(account_id = account_id))]
    pub async fn get_vehicles_stats(
        &self,
        account_id: AccountId,
    ) -> Result<Vec<VehicleStats>, WgError> {
        let url = url::Url::parse_with_params(
            "https://api.wotblitz.eu/wotb/tanks/stats/",
            &[
                ("application_id", self.application_id.as_str()),
                ("account_id", account_id.to_string().as_str()),
                ("fields", "tank_id,last_battle_time,all.battles"),
            ],
        )
        .context("failed to construct the URL")?;
        let result = self
            .client
            .get(url)
            .send()
            .await
            .with_context(|| format!("failed to retrieve player {account_id}'s vehicles stats"))?
            .json::<WgResponse<HashMap<String, Option<Vec<VehicleStats>>>>>()
            .await
            .with_context(|| format!("failed to parse player {account_id}'s vehicles stats"))?;
        match result {
            WgResponse::Ok { data } => Ok(data.into_values().next().flatten().unwrap_or_default()),
            WgResponse::Err { error } => Err(error),
        }
    }

    #[cfg(test)]
    pub async fn get_vehicles_stats(
        &self,
        _account_id: AccountId,
    ) -> Result<Vec<VehicleStats>, WgError> {
        let fake_non_played = VehicleStats {
            tank_id: 2,
            last_battle_time: Utc.timestamp_opt(0, 0).single(),
            inner: InnerVehicleStats { n_battles: 0 },
        };
        let fake_played = VehicleStats {
            tank_id: 1,
            last_battle_time: Utc.timestamp_opt(0, 0).single(),
            inner: InnerVehicleStats { n_battles: 1 },
        };
        Ok(vec![fake_played, fake_non_played])
    }

    /// [Log out][1].
    ///
    /// [1]: https://developers.wargaming.net/reference/all/wot/auth/logout/
    #[instrument(skip_all)]
    pub async fn log_out(&self, access_token: &str) -> Result<(), WgError> {
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
            .json::<WgResponse<()>>()
            .await
            .context("failed to parse the log-out response")?;
        match result {
            WgResponse::Ok { .. } => Ok(()),
            WgResponse::Err { error } => Err(error),
        }
    }
}

#[derive(Deserialize)]
pub struct AccountInfo {
    pub private: serde_json::Value,
}

/// Partial user's vehicle statistics.
#[derive(Deserialize)]
pub struct VehicleStats {
    pub tank_id: u16,

    #[serde(deserialize_with = "VehicleStats::deserialize_last_battle_time")]
    pub last_battle_time: Option<DateTime>,

    #[serde(rename = "all")]
    pub inner: InnerVehicleStats,
}

impl VehicleStats {
    pub const fn is_played(&self) -> bool {
        self.inner.n_battles != 0
    }

    /// Deserialize last battle time and take care of missing timestamps in the response.
    #[inline]
    fn deserialize_last_battle_time<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<DateTime>, D::Error> {
        let timestamp = i64::deserialize(deserializer)?;
        if timestamp == 0 {
            return Ok(None);
        }
        let Some(last_battle_time) = Utc.timestamp_opt(timestamp, 0).latest() else {
            return Ok(None);
        };
        Ok(Some(last_battle_time))
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
        serde_json::from_str::<WgResponse<HashMap<String, Vec<VehicleStats>>>>(
            // language=json
            r#"{"status":"ok","meta":{"count":1},"data":{"594778041":[{"all":{"battles":248},"last_battle_time":1681146251,"tank_id":18769}]}}"#,
        )?;
        Ok(())
    }

    #[test]
    fn parse_invalid_token_error_ok() -> Result {
        // language=json
        let error: WgError =
            serde_json::from_str(r#"{"code": 407, "message": "INVALID_ACCESS_TOKEN"}"#)?;
        match error {
            WgError::InvalidToken { .. } => Ok(()),
            _ => bail!(error),
        }
    }

    #[test]
    fn parse_other_error_ok() -> Result {
        // language=json
        let error: WgError = serde_json::from_str(r#"{"code": 418, "message": "I_AM_A_TEAPOT"}"#)?;
        match error {
            WgError::Api { code: 418, .. } => Ok(()),
            _ => bail!(error),
        }
    }
}
