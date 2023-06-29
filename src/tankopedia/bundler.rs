use std::{
    collections::BTreeMap,
    fs::{read, write, File},
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
        let parameters_path = self.data_path.join("3d").join("Tanks").join("Parameters");
        let mut vehicles: Vec<(VehicleDetails, VehicleParameters)> = stream::iter(Self::NATIONS)
            .then(|nation| {
                Self::load_nation(vehicles_path.join(nation), parameters_path.join(nation), &client)
            })
            .try_flatten()
            .try_collect()
            .await?;
        vehicles.sort_unstable_by_key(|(vehicle, _)| vehicle.tank_id);

        let mut module = File::options()
            .write(true)
            .create(true)
            .open(Path::new("src").join("tankopedia").join("vendored.rs"))?;
        let vendored_path = Path::new("src").join("tankopedia").join("vendored");

        writeln!(
            &mut module,
            "//! Auto-generated tankopedia, to update run `blitz-tanks bundle-tankopedia`.",
        )?;
        writeln!(&mut module)?;
        writeln!(&mut module, "use phf::{{phf_map, Map}};")?;
        writeln!(&mut module)?;
        writeln!(&mut module, "use crate::models::{{Vehicle, VehicleType}};")?;
        writeln!(&mut module)?;
        writeln!(&mut module, "pub static TANKOPEDIA: Map<u16, Vehicle> = phf_map! {{")?;
        for (details, parameters) in vehicles {
            writeln!(&mut module, "    {}_u16 => Vehicle {{", details.tank_id)?;
            writeln!(&mut module, "        tank_id: {:?},", details.tank_id)?;
            writeln!(&mut module, "        name: {:?},", details.user_string)?;
            writeln!(&mut module, "        tier: {:?},", details.tier)?;
            writeln!(&mut module, "        image_url: {:?},", details.image_url)?;
            writeln!(&mut module, "        is_premium: {:?},", details.is_premium)?;
            writeln!(&mut module, "        is_collectible: {:?},", details.is_collectible)?;
            writeln!(&mut module, "        type_: VehicleType::{:?},", details.type_)?;
            writeln!(&mut module, "    }},")?;

            self.copy_icon(
                details.tank_id,
                &parameters.resources_path.big_icon_path,
                &vendored_path,
            )?;
        }
        writeln!(&mut module, "}};")?;

        Ok(())
    }

    async fn load_nation(
        vehicles_path: PathBuf,
        parameters_path: PathBuf,
        client: &Client,
    ) -> Result<impl Stream<Item = Result<(VehicleDetails, VehicleParameters)>> + '_> {
        let path = vehicles_path.join("list.xml.dvpl");
        info!(?path, "📝 Unpacking…");
        let xml = unpack_dvpl(read(&path)?)?;
        let vehicles: BTreeMap<String, ()> = quick_xml::de::from_reader(xml.as_slice())?;
        let stream = stream::iter(vehicles).then(move |(vehicle_tag, _)| {
            Self::load_vehicle(client, parameters_path.clone(), vehicle_tag)
        });
        Ok(stream)
    }

    #[instrument(skip_all, fields(vehicle_tag = vehicle_tag))]
    async fn load_vehicle(
        client: &Client,
        parameters_path: PathBuf,
        vehicle_tag: String,
    ) -> Result<(VehicleDetails, VehicleParameters)> {
        info!("📤 Retrieving…");
        let vehicle = client
            .get(format!("https://eu.wotblitz.com/en/api/tankopedia/vehicle/{vehicle_tag}/"))
            .send()
            .await
            .with_context(|| format!("failed to request vehicle `{vehicle_tag}`"))?
            .json()
            .await
            .with_context(|| format!("failed to deserialize vehicle `{vehicle_tag}`"))?;
        let parameters: VehicleParameters = {
            let dvpl = read(parameters_path.join(&vehicle_tag).with_extension("yaml.dvpl"))?;
            serde_yaml::from_slice(&unpack_dvpl(dvpl)?)?
        };
        Ok((vehicle, parameters))
    }

    #[instrument(skip_all)]
    fn copy_icon(&self, tank_id: u16, big_icon_path: &str, vendored_path: &Path) -> Result {
        let big_icon_path = big_icon_path
            .strip_prefix("~res:/")
            .ok_or_else(|| anyhow!("incorrect icon path (`{}`)", big_icon_path))?;
        info!(big_icon_path, "📤 Copying…");
        let big_icon_path = self.data_path.join(format!("{big_icon_path}@2x.packed.webp.dvpl"));
        match read(&big_icon_path) {
            Ok(buffer) => {
                write(
                    vendored_path.join(tank_id.to_string()).with_extension("webp"),
                    unpack_dvpl(buffer)?,
                )?;
            }
            Err(error) => {
                warn!(?big_icon_path, "⚠️ Failed to read: {:#}", error);
            }
        }
        Ok(())
    }
}

#[must_use]
#[derive(Deserialize)]
struct VehicleDetails {
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

#[must_use]
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

#[must_use]
#[derive(Deserialize)]
struct VehicleParameters {
    #[serde(rename = "resourcesPath")]
    resources_path: ResourcesPath,
}

#[derive(Deserialize)]
struct ResourcesPath {
    #[serde(rename = "bigIconPath")]
    big_icon_path: String,
}
