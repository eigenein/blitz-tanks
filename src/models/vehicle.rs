use prost::Message;
use serde::Deserialize;

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
