use chrono::Utc;
use mongodb::bson::serde_helpers;
use serde::{Deserialize, Serialize};

use crate::{models::rating::Rating, prelude::*};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct VoteId {
    #[serde(rename = "aid")]
    pub account_id: u32,

    #[serde(rename = "tid")]
    pub tank_id: u16,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Vote {
    #[serde(rename = "_id")]
    pub id: VoteId,

    #[serde(
        rename = "ts",
        with = "serde_helpers::chrono_datetime_as_bson_datetime"
    )]
    pub timestamp: DateTime,

    #[serde(
        rename = "r",
        serialize_with = "Rating::serialize",
        deserialize_with = "Rating::deserialize"
    )]
    pub rating: Rating,
}

impl Vote {
    pub fn new(account_id: u32, tank_id: u16, rating: Rating) -> Self {
        Self {
            id: VoteId { account_id, tank_id },
            rating,
            timestamp: Utc::now(),
        }
    }
}
