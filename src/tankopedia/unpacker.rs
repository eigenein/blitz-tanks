use std::path::PathBuf;

use clap::Args;
use tokio::fs::write;
use walkdir::WalkDir;

use crate::{prelude::*, tankopedia::dvpl::Dvpl};

#[derive(Args)]
pub struct UnpackData {
    /// Path, inside which the DVPL's would be unpacked.
    #[clap()]
    path: PathBuf,
}

impl UnpackData {
    /// Unpack all the DVPL's in the specified directory recursively and place the
    /// unpacked files next to the original files.
    pub async fn run(self) -> Result {
        for entry in WalkDir::new(self.path) {
            let entry = entry?;
            let path = entry.path();
            if path
                .extension()
                .and_then(|extension| extension.to_str())
                .is_some_and(|extension| extension == "dvpl")
            {
                info!(?path, "📤 Unpacking…");
                let raw = Dvpl::read(path).await?.into_vec().await?;
                let path = path.with_extension("");
                info!(?path, "📥 Writing…");
                write(path, raw).await?;
            }
        }
        Ok(())
    }
}
