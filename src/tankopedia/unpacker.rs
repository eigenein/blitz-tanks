use std::{
    fs::{read, write},
    path::PathBuf,
};

use clap::Args;
use walkdir::WalkDir;

use crate::{prelude::*, tankopedia::dvpl::unpack_dvpl};

#[derive(Args)]
pub struct UnpackData {
    /// Path, inside which the DVPL's would be unpacked.
    #[clap()]
    path: PathBuf,
}

impl UnpackData {
    pub fn run(self) -> Result {
        for entry in WalkDir::new(self.path) {
            let entry = entry?;
            let path = entry.path();
            if path
                .extension()
                .and_then(|extension| extension.to_str())
                .is_some_and(|extension| extension == "dvpl")
            {
                info!(?path, "📤 Unpacking…");
                let raw = unpack_dvpl(read(path)?)?;
                let path = path.with_extension("");
                info!(?path, "📥 Writing…");
                write(path, raw)?;
            }
        }
        Ok(())
    }
}