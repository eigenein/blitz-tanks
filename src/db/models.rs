use clap::crate_version;
use mongodb::{
    bson::{doc, Bson},
    options::FindOneOptions,
    Collection, IndexModel,
};

use crate::{prelude::*, trainer::item_item::Model};

/// Collaborative filtering model repository.
#[derive(Clone)]
pub struct Models(Collection<Model>);

impl Models {
    pub async fn new(collection: Collection<Model>) -> Result<Self> {
        let index = IndexModel::builder().keys(doc! { "version": 1, "created_at": -1 }).build();
        collection.create_index(index, None).await?;
        Ok(Self(collection))
    }

    pub async fn insert(&self, model: &Model) -> Result<Bson> {
        Ok(self.0.insert_one(model, None).await?.inserted_id)
    }

    #[instrument(skip_all)]
    pub async fn get_latest(&self) -> Result<Model> {
        info!("📥 Loading the model…");
        let filter = doc! { "version": crate_version!() };
        let options = FindOneOptions::builder().sort(doc! { "created_at": -1 }).build();
        let model = self
            .0
            .find_one(filter, options)
            .await
            .context("failed to query the latest model")?
            .ok_or_else(|| {
                anyhow!(concat!(
                    "model is not found for version `",
                    crate_version!(),
                    "`, please re-run the trainer",
                ))
            })?;
        info!(%model.created_at, "✅ Loaded the model");
        Ok(model)
    }
}
