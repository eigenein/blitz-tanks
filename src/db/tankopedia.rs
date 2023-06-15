use std::collections::HashMap;

use mongodb::Collection;
use prost::Message;
use sled::Tree;
use url::Url;

use crate::{models::vehicle::VehicleDescription, prelude::*};

pub struct Tankopedia(Tree);

impl From<(Tree, Collection<VehicleDescription>)> for Tankopedia {
    fn from((tree, _collection): (Tree, Collection<VehicleDescription>)) -> Self {
        Self(tree)
    }
}

impl Tankopedia {
    /// Update the tankopedia database: insert new vehicles and update existing ones.
    pub fn update(&self, vehicles: Vec<VehicleDescription>) -> Result<&Self> {
        info!(n_vehicles = vehicles.len(), "📥 Updating the tankopedia…");
        for mut vehicle in vehicles {
            Self::fix_scheme(&mut vehicle)?;
            self.insert_vehicle(&vehicle)?;
        }
        Ok(self)
    }

    /// Insert the vehicles, which Wargaming.net is too lazy to add to the tankopedia.
    pub fn prepopulate(&self) -> Result<&Self> {
        info!("🤬 Pre-populating the tankopedia…");
        self.insert_unknown(9777, "WZ-114", true)?;
        self.insert_unknown(18241, "B-C Bourrasque", true)?;
        self.insert_unknown(12417, "Bisonte C45", true)?;
        self.insert_unknown(10545, "Wind", true)?;
        self.insert_unknown(24849, "Kryos", true)?;
        self.insert_unknown(20817, "Explorer", true)?;
        self.insert_unknown(1329, "Renault NC-31", false)?;
        self.insert_unknown(81, "Vickers Medium Mk. I", true)?;
        self.insert_unknown(3089, "Leichttraktor", true)?;
        self.insert_unknown(577, "Renault FT", true)?;
        self.insert_unknown(609, "R. Otsu", false)?;
        self.insert_unknown(545, "T1 Cunningham", true)?;
        self.insert_unknown(64081, "Mk I* Heavy Tank", true)?;
        self.insert_unknown(12673, "Bofors Tornvagn", true)?;
        self.insert_unknown(27425, "TL-7-120", true)?;
        self.insert_unknown(13441, "Aeonix", true)?;
        self.insert_unknown(25857, "Object 777 Version Ⅱ", true)?;
        self.insert_unknown(10609, "Magnate", true)?;
        self.insert_unknown(19777, "AltProto AMX 30", true)?;
        self.insert_unknown(26129, "Epsilon", true)?;
        self.insert_unknown(23297, "Object 244", true)?;
        self.insert_unknown(22353, "Churchill W", true)?;
        self.insert_unknown(20289, "Pirate", true)?;
        self.insert_unknown(10801, "Panlong", true)?;
        self.insert_unknown(10289, "WZ-132-1", false)?;
        Ok(self)
    }

    /// Load the tankopedia into a hashmap.
    pub fn load(&self) -> Result<HashMap<u16, VehicleDescription>> {
        info!("📤 Loading the tankopedia…");
        let tankopedia = self
            .0
            .iter()
            .map(|result| {
                let (key, value) = result?;
                Ok((
                    u16::from_be_bytes(key.as_ref().try_into()?),
                    VehicleDescription::decode(value.as_ref())?,
                ))
            })
            .collect::<Result<HashMap<u16, VehicleDescription>>>()
            .context("failed to load the tankopedia")?;
        info!(n_vehicles = tankopedia.len(), "✅ Loaded the tankopedia");
        Ok(tankopedia)
    }

    fn insert_vehicle(&self, vehicle: &VehicleDescription) -> Result {
        self.0.insert((vehicle.tank_id as u16).to_be_bytes(), vehicle.encode_to_vec())?;
        Ok(())
    }

    fn insert_unknown(&self, tank_id: u16, name: &str, is_premium: bool) -> Result {
        self.insert_vehicle(&VehicleDescription {
            tank_id: tank_id as u32,
            name: name.to_string(),
            images: Default::default(),
            is_premium,
        })
    }

    /// Wargaming is too lazy to use HTTPS either.
    fn fix_scheme(vehicle: &mut VehicleDescription) -> Result {
        if let Some(url) = &vehicle.images.normal_url {
            let mut url = Url::parse(url)?;
            url.set_scheme("https")
                .map_err(|_| anyhow!("failed to update scheme for #{}", vehicle.tank_id))?;
            vehicle.images.normal_url = Some(url.to_string());
        }
        Ok(())
    }
}
