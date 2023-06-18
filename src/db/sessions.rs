use std::time::Duration;

use mongodb::{
    bson::{doc, Bson},
    options::IndexOptions,
    Collection, IndexModel,
};
use uuid::Uuid;

use crate::{models::user::User, prelude::*};

/// Wrapper around the tree to manage client-side sessions.
#[derive(Clone)]
pub struct Sessions(Collection<User>);

impl Sessions {
    pub async fn new(collection: Collection<User>) -> Result<Self> {
        let options = IndexOptions::builder().expire_after(Duration::default()).build();
        let index = IndexModel::builder().keys(doc! { "expires_at": 1 }).options(options).build();
        collection
            .create_index(index, None)
            .await
            .context("failed to create the TTL index on sessions")?;
        Ok(Self(collection))
    }

    /// Insert the user to the session tree.
    #[instrument(level = "info", skip_all, fields(session_id = %user.session_id))]
    pub async fn insert(&self, user: &User) -> Result<Bson> {
        let inserted_id = self
            .0
            .insert_one(user, None)
            .await
            .with_context(|| format!("failed to insert the session `{}`", user.session_id))?
            .inserted_id;
        Ok(inserted_id)
    }

    #[cfg(test)]
    pub async fn insert_test_session(&self) -> Result<Uuid> {
        use chrono::Duration;

        let session_id = Uuid::new_v4();
        self.insert(&User {
            session_id,
            access_token: "test".to_string(),
            expires_at: Utc::now() + Duration::seconds(10),
            account_id: 0,
            nickname: "test".to_string(),
        })
        .await?;
        Ok(session_id)
    }

    /// Retrieve a user from the session tree.
    #[instrument(skip_all, level = "info", fields(session_id = %session_id), err)]
    pub async fn get(&self, session_id: Uuid) -> Result<Option<User>> {
        let user = self
            .0
            .find_one(doc! { "_id": session_id }, None)
            .await
            .with_context(|| format!("failed to retrieve session `{session_id}`"))?
            .filter(|user| user.expires_at > Utc::now());
        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use super::*;
    use crate::db::Db;

    #[tokio::test]
    async fn unknown_session_ok() -> Result {
        let session = Db::open_unittests().await?.sessions().await?.get(Uuid::new_v4()).await?;
        assert!(session.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn known_session_ok() -> Result {
        let manager = Db::open_unittests().await?.sessions().await?;
        let session_id = manager.insert_test_session().await?;
        let user = manager.get(session_id).await?;
        assert!(user.is_some());
        Ok(())
    }

    #[tokio::test]
    async fn expired_session_ok() -> Result {
        let manager = Db::open_unittests().await?.sessions().await?;
        let session_id = Uuid::new_v4();
        manager
            .insert(&User {
                session_id,
                access_token: "test".to_string(),
                expires_at: Utc::now() - Duration::seconds(10),
                account_id: 0,
                nickname: "test".to_string(),
            })
            .await?;
        let user = manager.get(session_id).await?;
        assert!(user.is_none(), "actual user: {user:?}");
        Ok(())
    }
}
