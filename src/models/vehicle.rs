pub struct Vehicle {
    pub tank_id: u16,
    pub name: &'static str,
    pub tier: u8,
    pub image_url: &'static str,
    pub is_premium: bool,
    pub is_collectible: bool,
    pub type_: VehicleType,
}

impl Vehicle {
    pub const fn is_premium(&self) -> bool {
        self.is_premium && !self.is_collectible
    }
}

pub enum VehicleType {
    Light,
    Medium,
    Heavy,
    AntiTank,
}
