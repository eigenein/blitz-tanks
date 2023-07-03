use std::{collections::HashMap, sync::Arc, time::Duration};

use reqwest::{Client, ClientBuilder};
use serde::Deserialize;
use tracing::instrument;

use crate::prelude::*;

/// Wargaming.net API client.
#[derive(Clone)]
pub struct Wg {
    client: Client,
    application_id: Arc<String>,
}

// TODO: perhaps, I need to extract `407/INVALID_ACCESS_TOKEN`.
#[derive(Deserialize, Debug, thiserror::Error)]
#[error("Wargaming.net API error {code}/{message}")]
pub struct WgError {
    pub code: u16,
    pub message: String,
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
pub enum NewWgError {
    #[error("invalid token")]
    InvalidToken {
        code: monostate::MustBe!(407),
        message: monostate::MustBe!("INVALID_ACCESS_TOKEN"),
    },

    #[error("Wargaming.net API error `{code}/{message}`")]
    Other { code: u16, message: String },
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
        account_id: u32,
        access_token: &str,
    ) -> Result<Option<AccountInfo>> {
        let url = url::Url::parse_with_params(
            "https://api.wotblitz.eu/wotb/account/info/",
            &[
                ("application_id", self.application_id.as_str()),
                ("account_id", account_id.to_string().as_str()),
                ("access_token", access_token),
                ("fields", "private"),
            ],
        )?;
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
            WgResponse::Err { error } => Err(error.into()),
        }
    }

    #[cfg(test)]
    pub async fn get_account_info(
        &self,
        _account_id: u32,
        _access_token: &str,
    ) -> Result<Option<AccountInfo>> {
        Ok(Some(AccountInfo {
            private: serde_json::Value::Object(Default::default()),
        }))
    }

    /// Get the accounts' vehicles' basic [statistics][1].
    ///
    /// [1]: https://developers.wargaming.net/reference/all/wotb/tanks/stats/
    #[cfg(not(test))]
    #[instrument(skip_all, fields(account_id = account_id))]
    pub async fn get_vehicles_stats(&self, account_id: u32) -> Result<Vec<VehicleStats>> {
        let url = url::Url::parse_with_params(
            "https://api.wotblitz.eu/wotb/tanks/stats/",
            &[
                ("application_id", self.application_id.as_str()),
                ("account_id", account_id.to_string().as_str()),
                ("fields", "tank_id,last_battle_time,all.battles"),
            ],
        )?;
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
            WgResponse::Err { error } => Err(error.into()),
        }
    }

    #[cfg(test)]
    pub async fn get_vehicles_stats(&self, _account_id: u32) -> Result<Vec<VehicleStats>> {
        let fake_non_played = VehicleStats {
            tank_id: 2,
            last_battle_time: Utc.timestamp_opt(0, 0).unwrap(),
            inner: InnerVehicleStats { n_battles: 0 },
        };
        let fake_played = VehicleStats {
            tank_id: 1,
            last_battle_time: Utc.timestamp_opt(0, 0).unwrap(),
            inner: InnerVehicleStats { n_battles: 1 },
        };
        Ok(vec![fake_played, fake_non_played])
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
            .json::<WgResponse<()>>()
            .await
            .context("failed to parse the log-out response")?;
        match result {
            WgResponse::Ok { .. } => Ok(()),
            WgResponse::Err { error } => Err(error.into()),
        }
    }
}

#[derive(Deserialize)]
pub struct AccountInfo {
    pub private: serde_json::Value,
}

/// Partial user's vehicle statistics.
#[serde_with::serde_as]
#[derive(Deserialize)]
pub struct VehicleStats {
    pub tank_id: u16,

    #[serde_as(as = "serde_with::TimestampSeconds<i64>")]
    pub last_battle_time: DateTime,

    #[serde(rename = "all")]
    pub inner: InnerVehicleStats,
}

impl VehicleStats {
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
        serde_json::from_str::<WgResponse<HashMap<String, Vec<VehicleStats>>>>(
            // language=json
            r#"{"status":"ok","meta":{"count":1},"data":{"594778041":[{"all":{"battles":248},"last_battle_time":1681146251,"tank_id":18769}]}}"#,
        )?;
        Ok(())
    }

    #[test]
    fn parse_invalid_token_error_ok() -> Result {
        // language=json
        let error: NewWgError =
            serde_json::from_str(r#"{"code": 407, "message": "INVALID_ACCESS_TOKEN"}"#)?;
        match error {
            NewWgError::InvalidToken { .. } => Ok(()),
            _ => bail!(error),
        }
    }

    #[test]
    fn parse_other_error_ok() -> Result {
        // language=json
        let error: NewWgError =
            serde_json::from_str(r#"{"code": 418, "message": "I_AM_A_TEAPOT"}"#)?;
        match error {
            NewWgError::Other { code: 418, .. } => Ok(()),
            _ => bail!(error),
        }
    }
}
