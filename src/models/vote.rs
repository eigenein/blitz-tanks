use chrono::Utc;
use mongodb::bson::serde_helpers;
use prost::Message;
use serde::{Deserialize, Serialize};

use crate::{models::rating::Rating, prelude::*};

/// User's vote for a vehicle.
#[derive(Message, Eq, PartialEq, Serialize)]
pub struct LegacyVote {
    #[prost(int64, tag = "1", required)]
    pub timestamp_secs: i64,

    #[prost(enumeration = "Rating", tag = "2", required)]
    pub rating: i32,
}

impl LegacyVote {
    pub fn new_now(rating: Rating) -> Self {
        Self {
            timestamp_secs: Utc::now().timestamp(),
            rating: rating as i32,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vote {
    pub account_id: u32,

    pub tank_id: i32,

    #[serde(with = "serde_helpers::chrono_datetime_as_bson_datetime")]
    pub timestamp: DateTime,

    #[serde(
        serialize_with = "Rating::serialize",
        deserialize_with = "Rating::deserialize"
    )]
    pub rating: Rating,
}
