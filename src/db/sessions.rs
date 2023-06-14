use chrono::Duration;
use mongodb::Collection;
use prost::Message;
use scru128::Scru128Id;
use sled::Tree;
use tracing::instrument;

use crate::{
    models::{LegacyUser, User},
    prelude::*,
};

/// Wrapper around the tree to manage client-side sessions.
#[derive(Clone)]
pub struct Sessions(Tree);

impl From<(Tree, Collection<User>)> for Sessions {
    fn from((tree, _collection): (Tree, Collection<User>)) -> Self {
        Self(tree)
    }
}

impl Sessions {
    /// Insert the user to the session tree.
    #[instrument(skip_all, fields(session_id = %user.session_id))]
    pub fn insert(&self, user: &User) -> Result {
        self.0
            .insert(user.session_id.to_bytes(), LegacyUser::from(user).encode_to_vec())
            .with_context(|| format!("failed to insert the session {:?}", user.session_id))?;
        Ok(())
    }

    #[cfg(test)]
    pub fn insert_test_session(&self) -> Result<Scru128Id> {
        let session_id = User::new_session_id();
        self.insert(&User {
            session_id,
            access_token: "test".to_string(),
            expires_at: Utc::now() + Duration::seconds(10),
            account_id: 0,
            nickname: "test".to_string(),
        })?;
        Ok(session_id)
    }

    /// Retrieve a user from the session tree.
    #[instrument(skip_all, fields(session_id = %session_id))]
    pub fn get(&self, session_id: Scru128Id) -> Result<Option<LegacyUser>> {
        let serialized_user = self
            .0
            .get(session_id.to_bytes())
            .with_context(|| format!("failed to retrieve session {session_id}"))?;
        let Some(serialized_user) = serialized_user else { return Ok(None) };
        let session = LegacyUser::decode(serialized_user.as_ref())
            .with_context(|| format!("failed to deserialize session {session_id}"))?;
        Ok((session.expires_at > Utc::now().timestamp()).then_some(session))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Db;

    #[tokio::test]
    #[ignore]
    async fn unknown_session_ok() -> Result {
        let session = Db::open_temporary().await?.session_manager()?.get(User::new_session_id())?;
        assert!(session.is_none());
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn known_session_ok() -> Result {
        let manager = Db::open_temporary().await?.session_manager()?;
        let session_id = manager.insert_test_session()?;
        let user = manager.get(session_id)?;
        assert!(user.is_some());
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn expired_session_ok() -> Result {
        let manager = Db::open_temporary().await?.session_manager()?;
        let session_id = User::new_session_id();
        manager.insert(&User {
            session_id,
            access_token: "test".to_string(),
            expires_at: Utc::now() - Duration::seconds(10),
            account_id: 0,
            nickname: "test".to_string(),
        })?;
        let user = manager.get(session_id)?;
        assert!(user.is_none(), "actual user: {user:?}");
        Ok(())
    }
}
