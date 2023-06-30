use std::{
    collections::BTreeMap,
    fs::{create_dir_all, File},
    io::{Cursor, Write},
    path::{Path, PathBuf},
};

use anyhow::bail;
use bytes::Bytes;
use clap::Args;
use futures::{stream, Stream, StreamExt, TryStreamExt};
use image::{DynamicImage, ImageFormat};
use img_parts::webp::WebP;
use reqwest::Client;
use serde::Deserialize;
use tokio::{fs::read, task::spawn_blocking};

use crate::{prelude::*, tankopedia::dvpl::unpack_dvpl};

#[derive(Args)]
pub struct BundleTankopedia {
    /// Path to the game client's `Data` directory.
    #[clap(
        long,
        default_value = "/Applications/World of Tanks Blitz.app/Contents/Resources/Data"
    )]
    data_path: PathBuf,

    /// Bundle the first vehicle from each nation (for testing and debugging).
    #[clap(long)]
    take_one: bool,
}

impl BundleTankopedia {
    /// Build the tankopedia from the game client and bundle it into the source code.
    ///
    /// Now, the game client is just a set of kludges, so… all hope abandon ye who enter here.
    pub async fn run(self) -> Result {
        let client = Client::new();

        static NATIONS: [&str; 9] = [
            "germany", "usa", "china", "france", "uk", "japan", "other", "european", "ussr",
        ];

        let vehicles_path = self.data_path.join("XML").join("item_defs").join("vehicles");
        let parameters_path = self.data_path.join("3d").join("Tanks").join("Parameters");
        let mut vehicles: Vec<_> = stream::iter(NATIONS)
            .then(|nation| {
                self.load_nation(vehicles_path.join(nation), parameters_path.join(nation), &client)
            })
            .try_flatten()
            .try_collect()
            .await?;
        vehicles.sort_unstable_by_key(|(vehicle, _)| vehicle.tank_id);

        let mut module = File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(Path::new("src").join("tankopedia").join("vendored.rs"))?;
        let vendored_path = Path::new("src").join("tankopedia").join("vendored");
        create_dir_all(&vendored_path)?;

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
        for (details, image) in vehicles {
            writeln!(&mut module, "    {}_u16 => Vehicle {{", details.tank_id)?;
            writeln!(&mut module, "        tank_id: {:?},", details.tank_id)?;
            writeln!(&mut module, "        name: {:?},", details.user_string)?;
            writeln!(&mut module, "        tier: {:?},", details.tier)?;
            writeln!(&mut module, "        type_: VehicleType::{:?},", details.type_)?;
            writeln!(&mut module, "        is_premium: {:?},", details.is_premium)?;
            writeln!(&mut module, "        is_collectible: {:?},", details.is_collectible)?;
            writeln!(&mut module, "        image_url: {:?},", details.image_url)?;
            if let Some(image) = image {
                let path = vendored_path.join(details.tank_id.to_string()).with_extension("webp");
                spawn_blocking(move || image.save(path)).await??;
                writeln!(
                    &mut module,
                    r#"        image_content: Some(include_bytes!("vendored/{}.webp")),"#,
                    details.tank_id
                )?;
            } else {
                writeln!(&mut module, "        image_content: None,")?;
            }
            writeln!(&mut module, r#"    }},"#)?;
        }
        writeln!(&mut module, "}};")?;

        Ok(())
    }

    async fn load_nation<'a>(
        &'a self,
        vehicles_path: PathBuf,
        parameters_path: PathBuf,
        client: &'a Client,
    ) -> Result<impl Stream<Item = Result<(VehicleDetails, Option<DynamicImage>)>> + 'a> {
        let path = vehicles_path.join("list.xml.dvpl");
        info!(?path, "📝 Unpacking…");
        let xml = unpack_dvpl(read(&path).await?).await?;
        let mut vehicles: BTreeMap<String, ()> = quick_xml::de::from_reader(xml.as_slice())?;
        if self.take_one {
            warn!("🐛 Stopping after the first vehicle");
            vehicles = vehicles.into_iter().take(1).collect();
        }
        let stream = stream::iter(vehicles).then(move |(vehicle_tag, _)| {
            self.load_vehicle(client, parameters_path.clone(), vehicle_tag)
        });
        Ok(stream)
    }

    #[instrument(skip_all, fields(vehicle_tag = vehicle_tag))]
    async fn load_vehicle(
        &self,
        client: &Client,
        parameters_path: PathBuf,
        vehicle_tag: String,
    ) -> Result<(VehicleDetails, Option<DynamicImage>)> {
        info!("📤 Retrieving…");
        let vehicle = client
            .get(format!("https://eu.wotblitz.com/en/api/tankopedia/vehicle/{vehicle_tag}/"))
            .send()
            .await
            .with_context(|| format!("failed to request vehicle `{vehicle_tag}`"))?
            .json()
            .await
            .with_context(|| format!("failed to deserialize vehicle `{vehicle_tag}`"))?;
        let image = {
            let dvpl = read(parameters_path.join(&vehicle_tag).with_extension("yaml.dvpl")).await?;
            let parameters: VehicleParameters = serde_yaml::from_slice(&unpack_dvpl(dvpl).await?)?;
            self.extract_vehicle_icon(&parameters.resources_path.big_icon_path).await?
        };
        Ok((vehicle, image))
    }

    /// Extract the vehicle icon from the game client.
    ///
    /// # Parameters
    ///
    /// - `big_icon_path`: the path coming from the vehicle parameters, it looks like
    ///   `~res:/Gfx/UI/BigTankIcons/ussr-KV_1s_BP`
    async fn extract_vehicle_icon(&self, big_icon_path: &str) -> Result<Option<DynamicImage>> {
        let big_icon_path = big_icon_path
            .strip_prefix("~res:/")
            .ok_or_else(|| anyhow!("unexpected icon path (`{}`)", big_icon_path))?;
        info!(big_icon_path, "📤 Copying…");
        let big_icon_path = self.data_path.join(format!("{big_icon_path}@2x.packed.webp.dvpl"));
        if !big_icon_path.exists() {
            return Ok(None);
        }
        let webp = unpack_dvpl(read(&big_icon_path).await?).await?;
        let (position_x, position_y, width, height) = Self::extract_dimensions(&webp)?;
        let image = image::io::Reader::with_format(Cursor::new(webp), ImageFormat::WebP)
            .decode()
            .with_context(|| format!("failed to decode `{big_icon_path:?}`"))?
            .crop(position_x, position_y, width, height);
        Ok(Some(image))
    }

    /// Extract dimensions from the WebP icon.
    ///
    /// # Returns
    ///
    /// - Position X
    /// - Position Y
    /// - Width
    /// - Height
    ///
    /// # Notes
    ///
    /// The metadata is included in the `extr` chunk of WebP icon and it looks like this:
    ///
    /// ```text
    /// 1
    ///
    /// 320 200
    /// 1
    /// 0 0 270 197 42 3 0 frame0
    /// ```
    fn extract_dimensions(webp: &[u8]) -> Result<(u32, u32, u32, u32)> {
        let webp =
            WebP::from_bytes(Bytes::copy_from_slice(webp)).context("failed to parse WebP")?;
        const EXTRA_ID: [u8; 4] = [101, 120, 116, 114];
        let chunk = webp
            .chunk_by_id(EXTRA_ID)
            .ok_or_else(|| anyhow!("the extra chunk is missing"))?
            .content()
            .data()
            .ok_or_else(|| anyhow!("no data in the extra chunk"))?;
        let chunk = String::from_utf8_lossy(chunk);
        let mut metadata = chunk.split_whitespace();
        let n_entries: usize = metadata
            .next()
            .ok_or_else(|| anyhow!("number of entries is missing"))?
            .parse()?;
        if n_entries != 1 {
            bail!("unexpected number of entries: {n_entries}");
        }
        metadata.next().ok_or_else(|| anyhow!("atlas width is missing"))?;
        metadata.next().ok_or_else(|| anyhow!("atlas height is missing"))?;
        let n_layers: usize =
            metadata.next().ok_or_else(|| anyhow!("number of layers is missing"))?.parse()?;
        if n_layers != 1 {
            bail!("unexpected number of layers: {n_layers}");
        }
        let position_x =
            metadata.next().ok_or_else(|| anyhow!("position X is missing"))?.parse()?;
        let position_y =
            metadata.next().ok_or_else(|| anyhow!("position Y is missing"))?.parse()?;
        let width = metadata.next().ok_or_else(|| anyhow!("width is missing"))?.parse()?;
        let height = metadata.next().ok_or_else(|| anyhow!("height is missing"))?.parse()?;
        Ok((position_x, position_y, width, height))
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
