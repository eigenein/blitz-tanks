pub struct Vehicle {
    pub tank_id: u16,
    pub name: &'static str,
    pub tier: u8,
    pub type_: VehicleType,
    pub is_premium: bool,
    pub is_collectible: bool,
    pub image_url: &'static str,
    pub image_content: &'static [u8],
}

impl Vehicle {
    pub const fn is_premium(&self) -> bool {
        self.is_premium && !self.is_collectible
    }
}

#[allow(unused)]
pub enum VehicleType {
    Light,
    Medium,
    Heavy,
    AntiTank,
}
