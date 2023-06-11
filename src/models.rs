//! Shared models which are used in both the database and the web app.

use cookie::{time::OffsetDateTime, Expiration};
use prost::Message;
use scru128::Scru128Id;
use serde::Deserialize;
use tracing::instrument;

use crate::prelude::*;

#[inline]
#[instrument(level = "debug", ret)]
pub fn new_session_id() -> Scru128Id {
    // SCRU128 is timestamp-based, so makes it easier to purge old sessions from the database.
    // It's also unpredictable, hence suitable for session IDs.
    scru128::new()
}

/// Authenticated [Wargaming.net user][1].
///
/// This model is used to parse the redirect parameters and store it in Sled.
///
/// [1]: https://developers.wargaming.net/reference/all/wot/auth/login/
#[serde_with::serde_as]
#[derive(Deserialize, Message)]
pub struct User {
    #[prost(string, tag = "1", required)]
    pub access_token: String,

    /// Expiration timestamp in seconds.
    #[serde_as(as = "serde_with::DisplayFromStr")]
    #[prost(int64, tag = "2", required)]
    pub expires_at: i64,

    #[serde_as(as = "serde_with::DisplayFromStr")]
    #[prost(uint32, tag = "3", required)]
    pub account_id: u32,

    #[prost(string, tag = "4", required)]
    pub nickname: String,
}

impl User {
    pub fn expires_at(&self) -> Result<Expiration> {
        Ok(Expiration::DateTime(OffsetDateTime::from_unix_timestamp(self.expires_at)?))
    }
}

/// Vehicle description from the [tankopedia][1].
///
/// This model is used to parse the API response and to store it in Sled.
///
/// [1]: https://developers.wargaming.net/reference/all/wotb/encyclopedia/vehicles/
#[derive(Deserialize, Message)]
pub struct VehicleDescription {
    /// # Notes
    ///
    /// Here I had to use [`u32`] because of [`prost`].
    #[prost(uint32, tag = "1", required)]
    pub tank_id: u32,

    #[prost(string, tag = "2", required)]
    pub name: String,

    #[prost(message, tag = "3", required)]
    pub images: VehicleImages,

    #[prost(bool, tag = "4", required)]
    pub is_premium: bool,
}

#[derive(Deserialize, Message)]
pub struct VehicleImages {
    #[prost(string, tag = "1", optional)]
    #[serde(rename = "normal")]
    pub normal_url: Option<String>,
}

/// User's rate action for a vehicle.
#[derive(Debug, prost::Enumeration)]
#[repr(i32)]
pub enum RateAction {
    Unknown = 0,
    Dislike = 1,
    Like = 2,
}

/// User's rating for a vehicle.
#[derive(Message)]
pub struct Rating {
    #[prost(int64, tag = "1", required)]
    pub timestamp_secs: i64,

    #[prost(enumeration = "RateAction", tag = "2", required)]
    pub action: i32,
}

impl Rating {
    pub fn new_now(action: RateAction) -> Self {
        Self {
            timestamp_secs: Utc::now().timestamp(),
            action: action as i32,
        }
    }
}
