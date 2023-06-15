use cookie::{time::OffsetDateTime, Expiration};
use mongodb::bson::serde_helpers;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::prelude::*;

/// Anonymous user.
pub struct Anonymous;

/// Authenticated [Wargaming.net user][1].
///
/// This model is used to parse the redirect parameters and store it in Sled.
///
/// [1]: https://developers.wargaming.net/reference/all/wot/auth/login/

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "_id", with = "serde_helpers::uuid_1_as_binary")]
    pub session_id: Uuid,

    pub account_id: u32,

    pub nickname: String,

    pub access_token: String,

    #[serde(with = "serde_helpers::chrono_datetime_as_bson_datetime")]
    pub expires_at: DateTime,
}

impl User {
    pub const SESSION_COOKIE_NAME: &'static str = "blitzTanksSessionId";

    pub fn expires_at(&self) -> Result<Expiration> {
        Ok(Expiration::DateTime(OffsetDateTime::from_unix_timestamp(
            self.expires_at.timestamp(),
        )?))
    }
}
