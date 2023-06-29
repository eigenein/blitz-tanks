pub struct Vehicle {
    pub name: &'static str,
    pub tier: u8,
    pub image_url: &'static str,
    pub is_premium: bool,
    pub is_collectible: bool,
    pub type_: VehicleType,
}

pub enum VehicleType {
    Light,
    Medium,
    Heavy,
    AntiTank,
}
