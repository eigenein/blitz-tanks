pub type Result<T = (), E = Error> = std::result::Result<T, E>;
pub use ::anyhow::{Context, Error};

#[allow(dead_code)]
pub type DateTime<Tz = Utc> = chrono::DateTime<Tz>;

pub use ::chrono::{TimeZone, Utc};
pub use anyhow::anyhow;
