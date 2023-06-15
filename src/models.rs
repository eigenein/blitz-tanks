//! Shared models which are used in both the database and the web app.

use cookie::{time::OffsetDateTime, Expiration};
use mongodb::bson::serde_helpers;
use prost::Message;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
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

/// User's rating for a vehicle.
#[derive(Debug, prost::Enumeration, PartialEq, Eq, Copy, Clone)]
#[repr(i32)]
pub enum Rating {
    #[deprecated]
    None = 0,

    Dislike = 1,
    Like = 2,
}

impl Rating {
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i32(match self {
            Self::None => 0,
            Self::Like => 1,
            Self::Dislike => 2,
        })
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let rating = i32::deserialize(deserializer)?;
        match rating {
            1 => Ok(Self::Like),
            2 => Ok(Self::Dislike),
            _ => Err(serde::de::Error::custom(format!("invalid rating value `{rating}`"))),
        }
    }
}

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
