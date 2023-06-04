pub mod session;

use std::path::PathBuf;

#[cfg(test)]
use sled::Config;
use tracing::instrument;

use crate::{db::session::SessionManager, prelude::*};

/// Convenience wrapper around the database.
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

    /// Open a temporary database for unit testing.
    #[cfg(test)]
    pub fn open_temporary() -> Result<Self> {
        Config::default()
            .temporary(true)
            .open()
            .context("failed to open a temporary database")
            .map(Into::into)
    }

    pub fn session_manager(&self) -> Result<SessionManager> {
        self.open_tree("sessions").map(Into::into)
    }

    fn open_tree(&self, name: &str) -> Result<sled::Tree> {
        self.0
            .open_tree(name)
            .with_context(|| format!("failed to open tree `{name}`"))
    }
}
