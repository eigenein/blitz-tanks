use mongodb::{bson::Bson, Collection};

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
}
