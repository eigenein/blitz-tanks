use std::collections::HashMap;

use futures::TryStreamExt;
use mongodb::{
    bson::{doc, to_document},
    options::{IndexOptions, UpdateOptions},
    Collection, IndexModel,
};

use crate::{models::vehicle::Vehicle, prelude::*};

pub struct Tankopedia(Collection<Vehicle>);

impl Tankopedia {
    pub async fn new(collection: Collection<Vehicle>) -> Result<Self> {
        let options = IndexOptions::builder().unique(true).build();
        let index = IndexModel::builder().keys(doc! { "tank_id": 1 }).options(options).build();
        collection
            .create_index(index, None)
            .await
            .context("failed to create the tankopedia index")?;
        Ok(Self(collection))
    }

    /// Update the tankopedia database: insert new vehicles and update existing ones.
    pub async fn update(&self, vehicles: Vec<Vehicle>) -> Result<&Self> {
        info!(n_vehicles = vehicles.len(), "📥 Updating the tankopedia…");
        for mut vehicle in vehicles {
            vehicle.images.fix_scheme()?;
            self.insert_vehicle(&vehicle).await?;
        }
        Ok(self)
    }

    /// Insert the vehicles, which Wargaming.net is too lazy to add to the tankopedia.
    pub async fn prepopulate(&self) -> Result<&Self> {
        info!("🤬 Pre-populating the tankopedia…");
        self.insert_unknown(9777, "WZ-114", true).await?;
        self.insert_unknown(18241, "B-C Bourrasque", true).await?;
        self.insert_unknown(12417, "Bisonte C45", true).await?;
        self.insert_unknown(10545, "Wind", true).await?;
        self.insert_unknown(24849, "Kryos", true).await?;
        self.insert_unknown(20817, "Explorer", true).await?;
        self.insert_unknown(1329, "Renault NC-31", false).await?;
        self.insert_unknown(81, "Vickers Medium Mk. I", true).await?;
        self.insert_unknown(3089, "Leichttraktor", true).await?;
        self.insert_unknown(577, "Renault FT", true).await?;
        self.insert_unknown(609, "R. Otsu", false).await?;
        self.insert_unknown(545, "T1 Cunningham", true).await?;
        self.insert_unknown(64081, "Mk I* Heavy Tank", true).await?;
        self.insert_unknown(12673, "Bofors Tornvagn", true).await?;
        self.insert_unknown(27425, "TL-7-120", true).await?;
        self.insert_unknown(13441, "Aeonix", true).await?;
        self.insert_unknown(25857, "Object 777 Version Ⅱ", true).await?;
        self.insert_unknown(10609, "Magnate", true).await?;
        self.insert_unknown(19777, "AltProto AMX 30", true).await?;
        self.insert_unknown(26129, "Epsilon", true).await?;
        self.insert_unknown(23297, "Object 244", true).await?;
        self.insert_unknown(22353, "Churchill W", true).await?;
        self.insert_unknown(20289, "Pirate", true).await?;
        self.insert_unknown(10801, "Panlong", true).await?;
        self.insert_unknown(10289, "WZ-132-1", false).await?;
        Ok(self)
    }

    /// Load the tankopedia into a hashmap.
    pub async fn load(&self) -> Result<HashMap<i32, Vehicle>> {
        info!("📤 Loading the tankopedia…");
        let tankopedia: HashMap<i32, Vehicle> = self
            .0
            .find(None, None)
            .await
            .context("failed to load the tankopedia")?
            .map_ok(|vehicle| (vehicle.tank_id, vehicle))
            .try_collect()
            .await?;
        info!(n_vehicles = tankopedia.len(), "✅ Loaded the tankopedia");
        Ok(tankopedia)
    }

    async fn insert_vehicle(&self, vehicle: &Vehicle) -> Result {
        let query = doc! { "tank_id": vehicle.tank_id };
        let options = UpdateOptions::builder().upsert(true).build();
        self.0
            .update_one(query, doc! { "$set": to_document(vehicle)? }, options)
            .await
            .with_context(|| format!("failed to upsert vehicle #{}", vehicle.tank_id))?;
        Ok(())
    }

    async fn insert_unknown(&self, tank_id: i32, name: &str, is_premium: bool) -> Result {
        let vehicle = Vehicle {
            tank_id,
            name: name.to_string(),
            images: Default::default(),
            is_premium,
        };
        self.insert_vehicle(&vehicle).await?;
        Ok(())
    }
}
