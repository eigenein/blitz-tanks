use std::ops::Sub;

use serde::{Deserialize, Deserializer, Serializer};

/// User's rating for a vehicle.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(i32)]
pub enum Rating {
    Dislike = 1,
    Like = 2,
}

impl Rating {
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i32(match self {
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

    #[inline]
    #[must_use]
    pub const fn into_f64(self) -> f64 {
        match self {
            Rating::Like => 0.5,
            Rating::Dislike => -0.5,
        }
    }
}

impl From<Rating> for f64 {
    #[inline]
    #[must_use]
    fn from(rating: Rating) -> f64 {
        rating.into_f64()
    }
}

impl Sub<f64> for Rating {
    type Output = f64;

    fn sub(self, rhs: f64) -> Self::Output {
        f64::from(self) - rhs
    }
}
