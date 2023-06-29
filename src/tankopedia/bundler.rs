use std::{
    collections::BTreeMap,
    fs::{read, File},
    io::Write,
    path::{Path, PathBuf},
};

use clap::Args;
use futures::{stream, Stream, StreamExt, TryStreamExt};
use reqwest::Client;
use serde::Deserialize;

use crate::{prelude::*, tankopedia::dvpl::unpack_dvpl};

#[derive(Args)]
pub struct BundleTankopedia {
    /// Path to the game client's `Data` directory.
    #[clap(
        long,
        default_value = "/Applications/World of Tanks Blitz.app/Contents/Resources/Data"
    )]
    data_path: PathBuf,
}

impl BundleTankopedia {
    const NATIONS: [&'static str; 9] = [
        "germany", "usa", "china", "france", "uk", "japan", "other", "european", "ussr",
    ];

    pub async fn run(self) -> Result {
        let client = Client::new();

        let vehicles_path = self.data_path.join("XML").join("item_defs").join("vehicles");
        let mut vehicles: Vec<Vehicle> = stream::iter(Self::NATIONS)
            .map(|nation| vehicles_path.join(nation))
            .then(|path| Self::stream_nation(path, &client))
            .try_flatten()
            .try_collect()
            .await?;
        vehicles.sort_unstable_by_key(|vehicle| vehicle.tank_id);

        let mut bundle = File::options()
            .write(true)
            .create(true)
            .open(Path::new("src").join("tankopedia").join("vendored.rs"))?;
        writeln!(
            &mut bundle,
            "//! Auto-generated tankopedia, to update run `blitz-tanks bundle-tankopedia`.",
        )?;
        writeln!(&mut bundle)?;
        writeln!(&mut bundle, "use phf::{{phf_map, Map}};")?;
        writeln!(&mut bundle)?;
        writeln!(&mut bundle, "use crate::tankopedia::{{Vehicle, VehicleType}};")?;
        writeln!(&mut bundle)?;
        writeln!(&mut bundle, "static TANKOPEDIA: Map<u16, Vehicle> = phf_map! {{")?;
        for vehicle in vehicles {
            writeln!(&mut bundle, "    {}_u16 => Vehicle {{", vehicle.tank_id)?;
            writeln!(&mut bundle, "        name: {:?},", vehicle.user_string)?;
            writeln!(&mut bundle, "        tier: {:?},", vehicle.tier)?;
            writeln!(&mut bundle, "        image_url: {:?},", vehicle.image_url)?;
            writeln!(&mut bundle, "        is_premium: {:?},", vehicle.is_premium)?;
            writeln!(&mut bundle, "        is_collectible: {:?},", vehicle.is_collectible)?;
            writeln!(&mut bundle, "        type_: VehicleType::{:?},", vehicle.type_)?;
            writeln!(&mut bundle, "    }},")?;
        }
        writeln!(&mut bundle, "}};")?;

        Ok(())
    }

    async fn stream_nation(
        root_path: PathBuf,
        client: &Client,
    ) -> Result<impl Stream<Item = Result<Vehicle>> + '_> {
        let path = root_path.join("list.xml.dvpl");
        info!(?path, "📝 Unpacking…");
        let xml = {
            let dvpl = read(&path).with_context(|| format!("failed to read `{path:?}`"))?;
            unpack_dvpl(dvpl).with_context(|| format!("failed to unpack `{path:?}`"))?
        };
        let vehicles: BTreeMap<String, ()> =
            quick_xml::de::from_reader(xml.as_slice()).context("failed to deserialize the XML")?;
        let stream = stream::iter(vehicles)
            .then(|(vehicle_tag, _)| Self::get_vehicle_details(client, vehicle_tag));
        Ok(stream)
    }

    #[instrument(skip_all, fields(vehicle_tag = vehicle_tag))]
    async fn get_vehicle_details(client: &Client, vehicle_tag: String) -> Result<Vehicle> {
        info!("☎️ Requesting…");
        client
            .get(format!("https://eu.wotblitz.com/en/api/tankopedia/vehicle/{vehicle_tag}/"))
            .send()
            .await
            .with_context(|| format!("failed to request vehicle `{vehicle_tag}`"))?
            .json()
            .await
            .with_context(|| format!("failed to deserialize vehicle `{vehicle_tag}`"))
    }
}

/// Game client's vehicle listing item. We only use it to parse the XMLs.
#[derive(Deserialize)]
struct Vehicle {
    #[serde(rename = "id")]
    tank_id: u16,

    #[serde(rename = "level")]
    tier: u8,

    #[serde(rename = "type_slug")]
    type_: VehicleType,

    user_string: String,
    image_url: String,
    is_premium: bool,
    is_collectible: bool,
}

#[derive(Debug, Deserialize)]
enum VehicleType {
    #[serde(rename = "lightTank")]
    Light,

    #[serde(rename = "mediumTank")]
    Medium,

    #[serde(rename = "heavyTank")]
    Heavy,

    #[serde(rename = "AT-SPG")]
    AntiTank,
}
