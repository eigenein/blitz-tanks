use mongodb::bson::Bson;

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    derive_more::Display,
    derive_more::From,
    derive_more::FromStr,
    derive_more::Into,
    serde::Deserialize,
    serde::Serialize,
)]
pub struct TankId(pub u16); // FIXME

impl From<TankId> for Bson {
    #[inline]
    fn from(tank_id: TankId) -> Self {
        Bson::from(tank_id.0 as i32)
    }
}
