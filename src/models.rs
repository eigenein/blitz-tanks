use chrono::TimeZone;
use prost::Message;
use serde::Deserialize;

use crate::prelude::*;

#[serde_with::serde_as]
#[derive(Deserialize, Message)]
pub struct User {
    #[prost(string, tag = "1")]
    access_token: String,

    #[serde_as(as = "serde_with::DisplayFromStr")]
    #[prost(int64, tag = "2")]
    expires_at: i64,

    #[serde_as(as = "serde_with::DisplayFromStr")]
    #[prost(uint32, tag = "3")]
    account_id: u32,

    #[prost(string, tag = "4")]
    nickname: String,
}

impl User {
    pub fn expires_at(&self) -> Result<DateTime> {
        Utc.timestamp_opt(self.expires_at, 0)
            .single()
            .with_context(|| format!("ambiguous timestamp: {}", self.expires_at))
    }
}
