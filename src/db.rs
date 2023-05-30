pub mod sessions;

use std::path::PathBuf;

use crate::db::sessions::Sessions;
use crate::prelude::*;

#[derive(Clone)]
pub struct Db(sled::Db);

impl From<sled::Db> for Db {
    fn from(db: sled::Db) -> Self {
        Self(db)
    }
}

impl Db {
    #[instrument(skip_all, fields(?path))]
    pub fn open(path: &PathBuf) -> Result<Self> {
        sled::open(path)
            .with_context(|| format!("failed to open the database from `{path:?}`"))
            .map(Into::into)
    }

    pub fn sessions(&self) -> Result<Sessions> {
        self.open_tree("sessions").map(Into::into)
    }

    fn open_tree(&self, name: &str) -> Result<sled::Tree> {
        self.0
            .open_tree(name)
            .with_context(|| format!("failed to open tree `{name}`"))
    }
}
