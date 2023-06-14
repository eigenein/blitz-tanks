pub mod sessions;
pub mod tankopedia;
pub mod votes;

use mongodb::{Collection, Database};
use sled::Tree;

use crate::{
    db::{sessions::Sessions, tankopedia::Tankopedia, votes::Votes},
    prelude::*,
};

/// Convenience wrapper around the database.
#[derive(Clone)]
pub struct Db {
    legacy_db: sled::Db,
    db: Database,
}

impl Db {
    pub const fn new(legacy_db: sled::Db, db: Database) -> Self {
        Self { legacy_db, db }
    }

    #[cfg(test)]
    pub async fn open_unittests() -> Result<Self> {
        use mongodb::Client;

        let legacy_db = sled::Config::default()
            .temporary(true)
            .open()
            .context("failed to open a temporary database")?;
        let db = Client::with_uri_str("mongodb://localhost").await?.database("unittests");
        db.drop(None)
            .await
            .context("failed to drop the database from the previous run")?;
        Ok(Self::new(legacy_db, db))
    }

    #[inline]
    pub async fn session_manager(&self) -> Result<Sessions> {
        Sessions::new(self.db.collection("sessions")).await
    }

    #[inline]
    pub fn tankopedia_manager(&self) -> Result<Tankopedia> {
        self.open_manager("tankopedia")
    }

    #[inline]
    pub fn vote_manager(&self) -> Result<Votes> {
        self.open_manager("ratings")
    }

    #[inline]
    pub fn open_manager<D, T: From<(Tree, Collection<D>)>>(&self, name: &str) -> Result<T> {
        let tree = self
            .legacy_db
            .open_tree(name)
            .with_context(|| format!("failed to open tree `{name}`"))?;
        let collection = self.db.collection(name);
        Ok(T::from((tree, collection)))
    }
}
