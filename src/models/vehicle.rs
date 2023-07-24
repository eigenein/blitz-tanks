use crate::models::TankId;

pub struct Vehicle {
    pub tank_id: TankId,
    pub name: &'static str,
    pub tier: u8,
    pub type_: VehicleType,
    pub availability: VehicleAvailability,
    pub image_content: &'static [u8],
}

#[allow(unused)]
pub enum VehicleType {
    Light,
    Medium,
    Heavy,
    AntiTank,
}

#[allow(unused)]
#[derive(Debug, Eq, PartialEq)]
pub enum VehicleAvailability {
    Researchable,
    Premium,
    Collectible,
}
