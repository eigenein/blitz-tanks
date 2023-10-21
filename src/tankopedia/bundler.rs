//! Tankopedia bundler.
//!
//! Compiles the vehicle details from the online tankopedia and game client to
//! build a consistent tankopedia with all the data included.
//!
//! Now, the game client is just a set of kludges, so… all hope abandon ye who enter here.

use std::{
    collections::{BTreeMap, HashMap},
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
use reqwest::{Client, StatusCode};
use serde::Deserialize;

use crate::{models::VehicleAvailability, prelude::*, tankopedia::dvpl::Dvpl};

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

    /// Skip saving the images, when only detail correction is needed.
    /// At the moment, it does not skip reading the images (TODO).
    #[clap(long)]
    skip_images: bool,
}

impl BundleTankopedia {
    /// Build the tankopedia from the game client and bundle it into the source code.
    pub async fn run(self) -> Result {
        let client = Client::new();

        let vehicles_path = self.data_path.join("XML").join("item_defs").join("vehicles");
        let parameters_path = self.data_path.join("3d").join("Tanks").join("Parameters");

        static NATIONS: [&str; 9] = [
            "germany", "usa", "china", "france", "uk", "japan", "other", "european", "ussr",
        ];
        let mut vehicles: Vec<_> = stream::iter(NATIONS)
            .then(|nation| {
                self.load_nation(vehicles_path.join(nation), parameters_path.join(nation), &client)
            })
            .try_flatten()
            .try_collect()
            .await?;

        // Sort the vehicles for pretty Git diffs when new vehicles are added.
        vehicles.sort_unstable_by_key(|(_, vehicle, _)| vehicle.tank_id);

        let translations: HashMap<String, String> = {
            let path = self.data_path.join("Strings").join("en.yaml.dvpl");
            serde_yaml::from_reader(Dvpl::read(path).await?.into_reader().await?)?
        };

        let mut module = File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(Path::new("src").join("tankopedia").join("vendored.rs"))?;
        let vendored_path = Path::new("src").join("tankopedia").join("vendored");
        create_dir_all(&vendored_path)?;

        if !self.skip_images {
            Self::save_images(&vendored_path, &vehicles)?;
        }

        writeln!(
            &mut module,
            "//! Auto-generated tankopedia, to update run `blitz-tanks bundle-tankopedia`.",
        )?;
        writeln!(&mut module)?;
        writeln!(
            &mut module,
            "use crate::models::{{TankId, Vehicle, VehicleAvailability::*, VehicleType::*}};"
        )?;
        writeln!(&mut module)?;
        Self::write_all_tank_ids(&mut module, &vehicles)?;
        writeln!(&mut module)?;
        Self::write_is_known_tank_id_function(&mut module, &vehicles)?;
        writeln!(&mut module)?;
        Self::write_get_vehicle_function(&mut module, &vehicles, &translations)?;

        Ok(())
    }

    #[instrument(skip_all)]
    fn save_images(
        vendored_path: &Path,
        vehicles: &[(VehicleXmlDetails, VehicleJsonDetails, DynamicImage)],
    ) -> Result {
        info!("📦 Saving images…");
        for (xml_details, json_details, image) in vehicles {
            info!(json_details.tank_id, xml_details.user_string_key);
            let path = vendored_path.join(json_details.tank_id.to_string()).with_extension("webp");
            image.save(path)?;
        }
        Ok(())
    }

    #[instrument(skip_all)]
    fn write_all_tank_ids(
        mut writer: impl Write,
        vehicles: &[(VehicleXmlDetails, VehicleJsonDetails, DynamicImage)],
    ) -> Result {
        info!("📦 Writing tank IDs…");
        writeln!(writer, "pub static ALL_TANK_IDS: [TankId; {}] = [", vehicles.len())?;
        for (_, json_details, _) in vehicles {
            writeln!(writer, "    TankId({}),", json_details.tank_id)?;
        }
        writeln!(writer, "];")?;
        Ok(())
    }

    #[instrument(skip_all)]
    fn write_is_known_tank_id_function(
        mut writer: impl Write,
        vehicles: &[(VehicleXmlDetails, VehicleJsonDetails, DynamicImage)],
    ) -> Result {
        info!("📦 Writing `is_known_tank_id()`…");
        writeln!(writer, "pub const fn is_known_tank_id(tank_id: TankId) -> bool {{")?;
        writeln!(writer, "    matches!(")?;
        writeln!(writer, "        tank_id,")?;
        for (i, (_, json_details, _)) in vehicles.iter().enumerate() {
            if i != 0 {
                writeln!(writer, "            | TankId({})", json_details.tank_id)?;
            } else {
                writeln!(writer, "        TankId({})", json_details.tank_id)?;
            }
        }
        writeln!(writer, "    )")?;
        writeln!(writer, "}}")?;
        Ok(())
    }

    #[instrument(skip_all)]
    fn write_get_vehicle_function(
        mut writer: impl Write,
        vehicles: &[(VehicleXmlDetails, VehicleJsonDetails, DynamicImage)],
        translations: &HashMap<String, String>,
    ) -> Result {
        info!("📦 Writing `get_vehicle()`…");
        writeln!(writer, "pub const fn get_vehicle(tank_id: TankId) -> Option<Vehicle> {{")?;
        writeln!(writer, "    match tank_id {{")?;
        for (xml_details, json_details, _) in vehicles {
            let short_user_string_key = xml_details.short_user_string_key();
            let name = translations
                // Take the short name from the client.
                .get(&short_user_string_key)
                // Ehm… sometimes the translation is the key itself, which doesn't make any sense.
                .filter(|translation| translation != &&short_user_string_key)
                // Fall back to the long name from the API.
                .unwrap_or(&json_details.user_string);

            writeln!(writer, "        TankId({}) => Some(Vehicle {{", json_details.tank_id)?;
            writeln!(writer, "            tank_id: TankId({:?}),", json_details.tank_id)?;
            writeln!(writer, "            name: {:?},", name)?;
            writeln!(writer, "            tier: {:?},", json_details.tier)?;
            writeln!(writer, "            type_: {:?},", json_details.type_)?;
            writeln!(
                writer,
                "            availability: {:?},",
                VehicleAvailability::from(json_details),
            )?;
            writeln!(
                writer,
                r#"            image_content: include_bytes!("vendored/{}.webp"),"#,
                json_details.tank_id
            )?;
            writeln!(writer, r#"        }}),"#)?;
        }
        writeln!(writer, "        _ => None,")?;
        writeln!(writer, "    }}")?;
        writeln!(writer, "}}")?;
        Ok(())
    }

    /// Load all vehicles of the specified nation.
    async fn load_nation<'a>(
        &'a self,
        vehicles_path: PathBuf,
        parameters_path: PathBuf,
        client: &'a Client,
    ) -> Result<
        impl Stream<Item = Result<(VehicleXmlDetails, VehicleJsonDetails, DynamicImage)>> + 'a,
    > {
        let path = vehicles_path.join("list.xml.dvpl");
        info!(?path, "📝 Unpacking…");
        let xml = Dvpl::read(&path).await?.into_vec().await?;
        let mut vehicles: BTreeMap<String, VehicleXmlDetails> =
            quick_xml::de::from_reader(xml.as_slice())?;
        if self.take_one {
            warn!("🐛 Stopping after the first vehicle");
            vehicles = vehicles.into_iter().take(1).collect();
        }
        let stream =
            stream::iter(vehicles)
                .map(Ok)
                .try_filter_map(move |(vehicle_tag, xml_details)| {
                    let parameters_path = parameters_path.clone();
                    async {
                        match self.load_vehicle(client, parameters_path, vehicle_tag).await? {
                            Some((json_details, image)) => {
                                Ok(Some((xml_details, json_details, image)))
                            }
                            None => Ok(None),
                        }
                    }
                });
        Ok(stream)
    }

    #[instrument(skip_all, fields(vehicle_tag = vehicle_tag))]
    async fn load_vehicle(
        &self,
        client: &Client,
        parameters_path: PathBuf,
        vehicle_tag: String,
    ) -> Result<Option<(VehicleJsonDetails, DynamicImage)>> {
        let details_url =
            format!("https://eu.wotblitz.com/en/api/tankopedia/vehicle/{vehicle_tag}/");
        info!(details_url, "📤 Retrieving details…");
        let response = client
            .get(&details_url)
            .send()
            .await
            .with_context(|| format!("failed to request vehicle `{vehicle_tag}`"))?;
        if response.status() == StatusCode::NOT_FOUND {
            warn!("⚠️ Vehicle JSON is not available");
            return Ok(None);
        }
        let details: VehicleJsonDetails = response
            .error_for_status()?
            .json()
            .await
            .with_context(|| format!("failed to deserialize vehicle `{vehicle_tag}`"))?;
        let image = {
            // First, try to request the image from the API.
            if let Ok(image) = Self::fetch_image(client, &details.image_url).await {
                Some(image)
            } else if let Ok(image) = {
                // Yeah, sometimes they return non-existing URLs, because… Wargaming, you know. Try some guess-work.
                let guessed_url = format!(
                    "https://glossary-eu-static.gcdn.co/icons/wotb/latest/uploaded/vehicles/hd/{vehicle_tag}.png"
                );
                Self::fetch_image(client, &guessed_url).await
            } {
                warn!(failed_url = details.image_url, "⚠️ Succeeded with the guessed URL");
                Some(image)
            } else {
                warn!(details.image_url, "⚠️ Falling back to the client icon");
                let dvpl =
                    Dvpl::read(parameters_path.join(&vehicle_tag).with_extension("yaml.dvpl"))
                        .await?;
                let parameters: VehicleParameters =
                    serde_yaml::from_reader(dvpl.into_reader().await?)?;
                self.extract_vehicle_icon(&parameters.resource_paths.big_icon_path).await?
            }
        };
        let Some(image) = image else {
            // This SHOULD NEVER happen. But this is Wargaming, so what would you expect?
            bail!("image is not found for `{vehicle_tag}`");
        };
        Ok(Some((details, image)))
    }

    #[instrument(skip_all, fields(url = url))]
    async fn fetch_image(client: &Client, url: &str) -> Result<DynamicImage> {
        let response = client.get(url).send().await?;
        if response.status() == StatusCode::OK {
            let raw = response.bytes().await?;
            Ok(image::io::Reader::new(Cursor::new(raw)).with_guessed_format()?.decode()?)
        } else {
            bail!("failed to fetch the image from {url} ({})", response.status());
        }
    }

    /// Extract the vehicle icon from the game client.
    ///
    /// # Parameters
    ///
    /// - `big_icon_path`: the path coming from the vehicle parameters, it looks like
    ///   `~res:/Gfx/UI/BigTankIcons/ussr-KV_1s_BP`
    #[instrument(skip_all, fields(big_icon_path = big_icon_path))]
    async fn extract_vehicle_icon(&self, big_icon_path: &str) -> Result<Option<DynamicImage>> {
        let big_icon_path = big_icon_path
            .strip_prefix("~res:/")
            .ok_or_else(|| anyhow!("unexpected icon path (`{}`)", big_icon_path))?;
        info!(big_icon_path, "📤 Extracting…");
        let big_icon_path = self.data_path.join(format!("{big_icon_path}@2x.packed.webp.dvpl"));
        if !big_icon_path.exists() {
            return Ok(None);
        }
        let webp = Dvpl::read(&big_icon_path).await?.into_vec().await?;
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

/// Vehicle details received from `https://eu.wotblitz.com/en/api/tankopedia/vehicle/*/`.
#[must_use]
#[derive(Deserialize)]
struct VehicleJsonDetails {
    #[serde(rename = "id")]
    tank_id: u16,

    #[serde(rename = "level")]
    tier: u8,

    #[serde(rename = "type_slug")]
    type_: VehicleType,

    /// This is a display name, not a translation key.
    user_string: String,

    image_url: String,
    is_premium: bool,
    is_collectible: bool,
}

impl From<&VehicleJsonDetails> for VehicleAvailability {
    #[inline]
    fn from(value: &VehicleJsonDetails) -> Self {
        if value.is_collectible {
            // They mark some collectibles as «premium», too. So, this order is important.
            Self::Collectible
        } else if value.is_premium {
            Self::Premium
        } else {
            Self::Researchable
        }
    }
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
    resource_paths: ResourcePaths,
}

#[derive(Deserialize)]
struct ResourcePaths {
    /// Path to the icon resource, for example: `~res:/Gfx/UI/BigTankIcons/ussr-KV_1s_BP`.
    #[serde(rename = "bigIconPath")]
    big_icon_path: String,
}

/// Vehicle details obtained from `list.xml.dvpl`.
#[must_use]
#[derive(Deserialize)]
struct VehicleXmlDetails {
    /// Example: `#ussr_vehicles:T-34`.
    #[serde(rename = "userString")]
    user_string_key: String,
}

impl VehicleXmlDetails {
    /// Get the translation key for the shortened vehicle name.
    /// This name is what you see in the vehicle ribbon in the game client.
    #[inline]
    pub fn short_user_string_key(&self) -> String {
        format!("{}_short", self.user_string_key)
    }
}
