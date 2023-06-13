use serde::Deserialize;

use crate::wg::error::WgError;

/// Wargaming.net API result.
#[derive(Deserialize)]
#[serde(tag = "status")]
pub enum WgResult<D> {
    #[serde(rename = "ok")]
    Ok { data: D },

    #[serde(rename = "error")]
    Err { error: WgError },
}
