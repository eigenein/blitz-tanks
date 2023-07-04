use std::time::Duration;

use clap::crate_version;
use mongodb::{
    bson::{doc, Bson},
    options::{FindOneOptions, IndexOptions},
    Collection, IndexModel,
};

use crate::{prelude::*, trainer::item_item::Model};

/// Collaborative filtering model repository.
#[derive(Clone)]
pub struct Models(Collection<Model>);

impl Models {
    pub async fn new(collection: Collection<Model>) -> Result<Self> {
        {
            let index = IndexModel::builder().keys(doc! { "version": 1, "created_at": -1 }).build();
            collection.create_index(index, None).await?;
        }
        {
            let expire_after = Duration::from_secs(7 * 24 * 60 * 60);
            let options = IndexOptions::builder().expire_after(expire_after).build();
            let index =
                IndexModel::builder().keys(doc! { "created_at": -1 }).options(options).build();
            collection
                .create_index(index, None)
                .await
                .context("failed to create the TTL index on models")?;
        }
        Ok(Self(collection))
    }

    #[instrument(skip_all, fields(version = model.version))]
    pub async fn insert(&self, model: &Model) -> Result<Bson> {
        let inserted_id = self.0.insert_one(model, None).await?.inserted_id;
        info!(?inserted_id, "🗃️ Model is inserted");
        Ok(inserted_id)
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
