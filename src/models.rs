use cookie::{time::OffsetDateTime, Expiration};
use prost::Message;
use serde::Deserialize;

use crate::prelude::*;

/// Authenticated [Wargaming.net user][1].
///
/// [1]: https://developers.wargaming.net/reference/all/wot/auth/login/
#[serde_with::serde_as]
#[derive(Deserialize, Message)]
pub struct User {
    #[prost(string, tag = "1")]
    pub access_token: String,

    /// Expiration timestamp in seconds.
    #[serde_as(as = "serde_with::DisplayFromStr")]
    #[prost(int64, tag = "2")]
    pub expires_at: i64,

    #[serde_as(as = "serde_with::DisplayFromStr")]
    #[prost(uint32, tag = "3")]
    pub account_id: u32,

    #[prost(string, tag = "4")]
    pub nickname: String,
}

impl User {
    pub fn expires_at(&self) -> Result<Expiration> {
        Ok(Expiration::DateTime(OffsetDateTime::from_unix_timestamp(self.expires_at)?))
    }
}
