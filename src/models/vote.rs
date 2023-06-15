use chrono::Utc;
use prost::Message;
use serde::Serialize;

use crate::models::rating::Rating;

/// User's vote for a vehicle.
#[derive(Message, Eq, PartialEq, Serialize)]
pub struct Vote {
    #[prost(int64, tag = "1", required)]
    pub timestamp_secs: i64,

    #[prost(enumeration = "Rating", tag = "2", required)]
    pub rating: i32,
}

impl Vote {
    pub fn new_now(rating: Rating) -> Self {
        Self {
            timestamp_secs: Utc::now().timestamp(),
            rating: rating as i32,
        }
    }
}
