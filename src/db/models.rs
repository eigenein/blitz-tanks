use mongodb::{
    bson::{doc, Bson},
    options::FindOneOptions,
    Collection,
};

use crate::{prelude::*, trainer::item_item::Model};

/// Collaborative filtering model repository.
#[derive(Clone)]
pub struct Models(Collection<Model>);

impl Models {
    pub async fn new(collection: Collection<Model>) -> Result<Self> {
        Ok(Self(collection))
    }

    pub async fn insert(&self, model: &Model) -> Result<Bson> {
        Ok(self.0.insert_one(model, None).await?.inserted_id)
    }

    #[instrument(skip_all)]
    pub async fn get_latest(&self) -> Result<Option<Model>> {
        info!("📥 Loading the model…");
        let options = FindOneOptions::builder().sort(doc! { "_id": -1 }).build();
        self.0
            .find_one(None, options)
            .await
            .context("failed to load the latest model (may need a refit)")
    }
}
