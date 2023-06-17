use serde::{Deserialize, Serialize};
use url::Url;

use crate::prelude::*;

/// Vehicle description from the [tankopedia][1].
///
/// [1]: https://developers.wargaming.net/reference/all/wotb/encyclopedia/vehicles/
#[derive(Serialize, Deserialize, Debug)]
pub struct Vehicle {
    pub tank_id: i32,
    pub name: String,
    pub images: VehicleImages,
    pub is_premium: bool,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct VehicleImages {
    #[serde(rename = "normal")]
    pub normal_url: Option<String>,
}

impl VehicleImages {
    /// Wargaming is too lazy to update the URLs to use HTTPS.
    pub(crate) fn fix_scheme(&mut self) -> Result<&mut Self> {
        if let Some(url) = &self.normal_url {
            let mut url = Url::parse(url)?;
            url.set_scheme("https")
                .map_err(|_| anyhow!("failed to update the URL scheme"))?;
            self.normal_url = Some(url.to_string());
        }
        Ok(self)
    }
}
