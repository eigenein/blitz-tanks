pub mod models;
pub mod sessions;
pub mod tankopedia;
pub mod votes;

use mongodb::Database;

use crate::{
    db::{sessions::Sessions, tankopedia::Tankopedia, votes::Votes},
    prelude::*,
};

/// Convenience wrapper around the database.
#[derive(Clone, derive_more::From)]
pub struct Db(Database);

impl Db {
    #[cfg(test)]
    pub async fn open_unittests() -> Result<Self> {
        use mongodb::{options::ClientOptions, Client};
        let db = Client::with_options(ClientOptions::default())?.database("unittests");
        db.drop(None)
            .await
            .context("failed to drop the database from the previous run")?;
        Ok(db.into())
    }

    #[inline]
    pub async fn sessions(&self) -> Result<Sessions> {
        Sessions::new(self.0.collection("sessions")).await
    }

    #[inline]
    pub async fn tankopedia(&self) -> Result<Tankopedia> {
        Tankopedia::new(self.0.collection("tankopedia")).await
    }

    #[inline]
    pub async fn votes(&self) -> Result<Votes> {
        Votes::new(self.0.collection("votes")).await
    }

    // #[inline]
    // pub async fn models(&self) -> Result<Models> {
    //     Models::new(self.0.collection("models")).await
    // }
}
