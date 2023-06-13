use serde::Deserialize;

use crate::prelude::*;

#[derive(Deserialize)]
pub struct WgError {
    pub code: u16,
    pub message: String,
}

impl From<WgError> for Error {
    fn from(error: WgError) -> Self {
        anyhow!("Wargaming.net API error #{} ({})", error.code, error.message)
    }
}
