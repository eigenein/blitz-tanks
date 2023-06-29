use std::{
    collections::BTreeMap,
    fs::{read, File},
    io::Write,
    path::{Path, PathBuf},
};

use clap::Args;
use serde::Deserialize;

use crate::{
    prelude::*,
    tankopedia::{dvpl::unpack_dvpl, Vehicle},
};

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
    pub fn run(self) -> Result {
        let vehicles_path = self.data_path.join("XML").join("item_defs").join("vehicles");
        let vehicles: BTreeMap<u16, ClientVehicle> =
            Self::iter_nation(&vehicles_path.join("ussr"), 0)?
                .chain(Self::iter_nation(&vehicles_path.join("germany"), 1)?)
                .chain(Self::iter_nation(&vehicles_path.join("usa"), 2)?)
                .chain(Self::iter_nation(&vehicles_path.join("china"), 3)?)
                .chain(Self::iter_nation(&vehicles_path.join("france"), 4)?)
                .chain(Self::iter_nation(&vehicles_path.join("uk"), 5)?)
                .chain(Self::iter_nation(&vehicles_path.join("japan"), 6)?)
                .chain(Self::iter_nation(&vehicles_path.join("other"), 7)?)
                .chain(Self::iter_nation(&vehicles_path.join("european"), 8)?)
                .collect();
        let mut bundle = File::options()
            .write(true)
            .create(true)
            .open(Path::new("src").join("tankopedia").join("vendored.rs"))?;

        writeln!(
            &mut bundle,
            "//! Auto-generated tankopedia, to update run `blitz-tanks bundle-tankopedia`.",
        )?;
        writeln!(&mut bundle)?;
        writeln!(&mut bundle, "use phf::{{Map, phf_map}};")?;
        writeln!(&mut bundle)?;
        writeln!(&mut bundle, "use crate::tankopedia::Vehicle;")?;
        writeln!(&mut bundle)?;
        writeln!(&mut bundle, "#[rustfmt::skip]")?;
        writeln!(&mut bundle, "static TANKOPEDIA: Map<u16, Vehicle> = phf_map! {{")?;
        for (tank_id, vehicle) in vehicles {
            let vehicle = Vehicle { tier: vehicle.tier };
            writeln!(&mut bundle, "    {tank_id}_u16 => {:?},", vehicle)?;
        }
        writeln!(&mut bundle, "}};")?;

        Ok(())
    }

    fn iter_nation(
        root_path: &Path,
        nation_id: u16,
    ) -> Result<impl Iterator<Item = (u16, ClientVehicle)>> {
        let path = root_path.join("list.xml.dvpl");
        info!(?path, "📝 Unpacking…");
        let dvpl = read(&path).with_context(|| format!("failed to read `{path:?}`"))?;
        let xml = unpack_dvpl(dvpl).with_context(|| format!("failed to unpack `{path:?}`"))?;
        let vehicles: BTreeMap<String, ClientVehicle> =
            quick_xml::de::from_reader(xml.as_slice()).context("failed to deserialize the XML")?;
        Ok(vehicles
            .into_values()
            .map(move |vehicle| (vehicle.make_tank_id(nation_id), vehicle)))
    }
}

/// Game client's vehicle listing item. We only use it to parse the XMLs.
#[derive(Deserialize)]
struct ClientVehicle {
    id: u16,

    #[serde(rename = "userString")]
    user_string: String,

    #[serde(rename = "level")]
    tier: u8,

    tags: Vec<Tag>,
}

impl ClientVehicle {
    #[inline]
    pub const fn make_tank_id(&self, nation_id: u16) -> u16 {
        (self.id << 8) + (nation_id << 4) + 1
    }
}

#[derive(Deserialize)]
enum Tag {
    #[serde(rename = "lightTank")]
    LightTank,

    #[serde(rename = "heavyTank")]
    HeavyTank,

    #[serde(rename = "mediumTank")]
    MediumTank,

    #[serde(rename = "AT-SPG")]
    AntiTank,

    #[serde(rename = "collectible")]
    Collectible,

    #[serde(other)]
    Other,
}
