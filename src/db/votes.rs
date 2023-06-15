use mongodb::{
    bson::{doc, to_document},
    options::{IndexOptions, UpdateOptions},
    Collection, IndexModel,
};
use prost::Message;
use sled::Tree;

use crate::{
    models::vote::{LegacyVote, Vote},
    prelude::*,
};

#[derive(Clone)]
pub struct Votes(Tree, Collection<Vote>);

impl Votes {
    pub async fn new(tree: Tree, collection: Collection<Vote>) -> Result<Self> {
        let options = IndexOptions::builder().unique(true).build();
        let index = IndexModel::builder()
            .keys(doc! { "account_id": 1, "tank_id": 1 })
            .options(options)
            .build();
        collection
            .create_index(index, None)
            .await
            .context("failed to create index on votes")?;
        Ok(Self(tree, collection))
    }

    #[instrument(skip_all, fields(account_id = account_id, tank_id = tank_id))]
    pub async fn insert(&self, account_id: u32, tank_id: u16, vote: &LegacyVote) -> Result {
        self.0
            .insert(Self::encode_key(account_id, tank_id), vote.encode_to_vec())
            .with_context(|| format!("failed to insert the #{account_id}'s vote for #{tank_id}"))?;
        self.insert_new(account_id, tank_id, vote).await?;
        Ok(())
    }

    pub async fn insert_new(
        &self,
        account_id: u32,
        tank_id: u16,
        legacy_vote: &LegacyVote,
    ) -> Result {
        let vote = &Vote {
            account_id,
            tank_id: tank_id as i32,
            timestamp: Utc
                .timestamp_opt(legacy_vote.timestamp_secs, 0)
                .single()
                .ok_or_else(|| anyhow!("failed to convert the timestamp"))?,
            rating: legacy_vote.rating(),
        };
        let query = doc! {
            "account_id": vote.account_id,
            "tank_id": vote.tank_id,
            "timestamp": { "$lte": vote.timestamp },
        };
        let options = UpdateOptions::builder().upsert(true).build();
        self.1
            .update_one(query, doc! { "$set": to_document(vote)? }, options)
            .await
            .with_context(|| format!("failed to upsert `{vote:?}`"))?;
        Ok(())
    }

    /// Retrieve a single vote.
    #[instrument(skip_all, fields(account_id = account_id, tank_id = tank_id))]
    pub fn get(&self, account_id: u32, tank_id: u16) -> Result<Option<LegacyVote>> {
        self.0
            .get(Self::encode_key(account_id, tank_id))?
            .map(|value| LegacyVote::decode(value.as_ref()))
            .transpose()
            .with_context(|| format!("failed to retrieve a #{account_id}'s vote for #{tank_id}"))
    }

    #[instrument(skip_all, fields(account_id = account_id, tank_id = tank_id))]
    pub fn delete(&self, account_id: u32, tank_id: u16) -> Result {
        self.0.remove(Self::encode_key(account_id, tank_id))?;
        Ok(())
    }

    /// Retrieve all votes of the user.
    #[instrument(skip_all, fields(account_id = account_id))]
    pub fn get_all_by_account_id(&self, account_id: u32) -> Result<Vec<(u16, LegacyVote)>> {
        self.0
            .scan_prefix(account_id.to_be_bytes())
            .map(|result| {
                let (key, value) = result?;
                let tank_id = Self::decode_tank_id(key.as_ref())?;
                let event = LegacyVote::decode(value.as_ref())?;
                Ok((tank_id, event))
            })
            .collect()
    }

    /// Iterate over **all** the votes.
    pub fn iter_all(&self) -> impl Iterator<Item = Result<(u32, u16, LegacyVote)>> {
        self.0.iter().map(|result| {
            let (key, value) = result?;
            let (account_id, tank_id) = Self::decode_key(key.as_ref())?;
            Ok((account_id, tank_id, LegacyVote::decode(value.as_ref())?))
        })
    }

    /// Encode the key corresponding to the user's vehicle.
    ///
    /// # Considerations
    ///
    /// 1. Key must be sortable, hence the big-endian encoding.
    /// 2. I should be able to retrieve all user's votes in one go, hence keys start with account ID.
    /// 3. I should be able to retrieve individual votes, hence the key contains tank ID as well.
    #[inline]
    fn encode_key(account_id: u32, tank_id: u16) -> Vec<u8> {
        [&account_id.to_be_bytes()[..], &tank_id.to_be_bytes()[..]].concat()
    }

    /// Decode tank ID from the Sled key.
    #[inline]
    fn decode_tank_id(key: &[u8]) -> Result<u16> {
        Ok(u16::from_be_bytes((&key[4..6]).try_into()?))
    }

    #[inline]
    fn decode_key(key: &[u8]) -> Result<(u32, u16)> {
        let account_id = u32::from_be_bytes((&key[0..4]).try_into()?);
        Ok((account_id, Self::decode_tank_id(key)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{db::Db, models::rating::Rating};

    #[tokio::test]
    async fn insert_get_vote_ok() -> Result {
        let manager = Db::open_unittests().await?.vote_manager().await?;
        manager.insert(1, 42, &LegacyVote::new_now(Rating::Like)).await?;
        assert!(manager.get(1, 42)?.is_some());
        assert_eq!(manager.get(2, 42)?, None);
        assert_eq!(manager.get(42, 1)?, None);
        Ok(())
    }

    #[tokio::test]
    async fn get_all_by_account_id_ok() -> Result {
        let manager = Db::open_unittests().await?.vote_manager().await?;
        let vote = LegacyVote::new_now(Rating::Like);
        manager.insert(1, 42, &vote).await?;
        assert_eq!(manager.get_all_by_account_id(0)?, []);
        assert_eq!(manager.get_all_by_account_id(1)?, [(42, vote)]);
        assert_eq!(manager.get_all_by_account_id(2)?, []);
        Ok(())
    }

    #[tokio::test]
    async fn delete_vote_ok() -> Result {
        let manager = Db::open_unittests().await?.vote_manager().await?;
        manager.insert(1, 42, &LegacyVote::new_now(Rating::Like)).await?;
        manager.delete(1, 42)?;
        assert_eq!(manager.get(1, 42)?, None);
        Ok(())
    }
}
