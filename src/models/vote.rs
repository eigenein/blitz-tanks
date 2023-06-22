use chrono::Utc;
use mongodb::bson::serde_helpers;
use serde::{Deserialize, Serialize};

use crate::{models::rating::Rating, prelude::*};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Vote {
    pub account_id: u32,

    pub tank_id: u16,

    #[serde(with = "serde_helpers::chrono_datetime_as_bson_datetime")]
    pub timestamp: DateTime,

    #[serde(
        serialize_with = "Rating::serialize",
        deserialize_with = "Rating::deserialize"
    )]
    pub rating: Rating,
}

impl Vote {
    pub fn new(account_id: u32, tank_id: u16, rating: Rating) -> Self {
        Self {
            account_id,
            tank_id,
            rating,
            timestamp: Utc::now(),
        }
    }
}
