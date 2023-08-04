use chrono::Utc;
use mongodb::bson::serde_helpers;
use serde::{Deserialize, Serialize};

use crate::{
    models::{rating::Rating, AccountId, TankId},
    prelude::*,
};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct VoteId {
    #[serde(rename = "aid")]
    pub account_id: AccountId,

    #[serde(rename = "tid")]
    pub tank_id: TankId,
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

    #[serde(rename = "nb", default)]
    pub n_battles: Option<u32>,
}

impl Vote {
    pub fn new(
        account_id: impl Into<AccountId>,
        tank_id: TankId,
        n_battles: u32,
        rating: Rating,
    ) -> Self {
        Self {
            id: VoteId {
                account_id: account_id.into(),
                tank_id,
            },
            n_battles: Some(n_battles),
            rating,
            timestamp: Utc::now(),
        }
    }
}
