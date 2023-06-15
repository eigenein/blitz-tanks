use serde::{Deserialize, Deserializer, Serializer};

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
