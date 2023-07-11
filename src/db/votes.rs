use futures::TryStreamExt;
use itertools::Itertools;
use mongodb::{
    bson::{doc, to_document},
    options::UpdateOptions,
    Collection, Cursor, IndexModel,
};

use crate::{
    models::{Vote, VoteId},
    prelude::*,
    tankopedia::vendored::TANKOPEDIA,
};

#[derive(Clone)]
pub struct Votes(Collection<Vote>);

impl Votes {
    pub async fn new(collection: Collection<Vote>) -> Result<Self> {
        let index = IndexModel::builder().keys(doc! { "_id.aid": 1 }).build();
        collection.create_index(index, None).await?;
        Ok(Self(collection))
    }

    #[instrument(skip_all, fields(account_id = vote.id.account_id, tank_id = vote.id.tank_id))]
    pub async fn insert(&self, vote: &Vote) -> Result {
        let options = UpdateOptions::builder().upsert(true).build();
        self.0
            .update_one(
                doc! { "_id": to_document(&vote.id)? },
                doc! { "$set": to_document(vote)? },
                options,
            )
            .await
            .with_context(|| {
                format!("failed to upsert #{}'s vote for #{}", vote.id.account_id, vote.id.tank_id)
            })?;
        Ok(())
    }

    #[instrument(skip_all, fields(account_id = account_id, tank_id = tank_id))]
    pub async fn delete(&self, account_id: u32, tank_id: u16) -> Result {
        let vote_id = VoteId { account_id, tank_id };
        self.0
            .delete_one(doc! { "_id": to_document(&vote_id)? }, None)
            .await
            .with_context(|| format!("failed to remove #{account_id}'s vote for #{tank_id}"))?;
        Ok(())
    }

    /// Retrieve all votes of the user.
    #[instrument(skip_all, fields(account_id = account_id))]
    pub async fn iter_by_account_id(&self, account_id: u32) -> Result<Cursor<Vote>> {
        self.0
            .find(doc! { "_id.aid": account_id }, None)
            .await
            .with_context(|| format!("failed to query #{account_id}'s votes"))
    }

    /// Iterate over **all** the votes. Only the **tankopedia vehicles** are taken into account.
    pub async fn iter_all(&self) -> Result<Cursor<Vote>> {
        let filter = doc! { "_id.tid": { "$in": TANKOPEDIA.keys().map(|tank_id| *tank_id as u32).collect_vec() } };
        self.0.find(filter, None).await.context("failed to query all votes")
    }

    #[inline]
    pub async fn retrieve_all(&self) -> Result<Vec<Vote>> {
        Ok(self.iter_all().await?.try_collect().await?)
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, DurationRound};
    use futures::TryStreamExt;

    use super::*;
    use crate::{db::Db, models::Rating};

    #[tokio::test]
    #[ignore]
    async fn get_all_by_account_id_ok() -> Result {
        let manager = Db::open_unittests().await?.votes().await?;
        let mut vote = Vote::new(1, 42, Rating::Like);
        vote.timestamp = vote.timestamp.duration_round(Duration::seconds(1))?;
        manager.insert(&vote).await?;

        assert_eq!(manager.iter_by_account_id(0).await?.try_collect::<Vec<Vote>>().await?, []);
        assert_eq!(manager.iter_by_account_id(1).await?.try_collect::<Vec<Vote>>().await?, [vote]);
        assert_eq!(manager.iter_by_account_id(2).await?.try_collect::<Vec<Vote>>().await?, []);

        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn delete_vote_ok() -> Result {
        let manager = Db::open_unittests().await?.votes().await?;
        let vote = Vote::new(1, 42, Rating::Like);
        manager.insert(&vote).await?;
        manager.delete(1, 42).await?;
        assert!(
            manager
                .iter_by_account_id(1)
                .await?
                .try_collect::<Vec<Vote>>()
                .await?
                .is_empty()
        );
        Ok(())
    }
}
