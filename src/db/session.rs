//! Database's session manager. Do not confuse with the session cookie manager.

use prost::Message;
use scru128::Scru128Id;
use sled::Tree;
use tracing::instrument;

use crate::{models::User, prelude::*};

/// Wrapper around the tree to manage client-side sessions.
#[derive(Clone)]
pub struct SessionManager(Tree);

impl From<Tree> for SessionManager {
    fn from(tree: Tree) -> Self {
        Self(tree)
    }
}

impl SessionManager {
    /// Insert the user to the session tree.
    #[instrument(skip_all, fields(session_id = %session_id))]
    pub fn insert(&self, session_id: Scru128Id, user: &User) -> Result {
        self.0
            .insert(session_id.to_bytes(), user.encode_to_vec())
            .with_context(|| format!("failed to insert the session {session_id:?}"))?;
        Ok(())
    }

    /// Retrieve a user from the session tree.
    #[instrument(skip_all, fields(session_id = %session_id))]
    pub fn get(&self, session_id: Scru128Id) -> Result<Option<User>> {
        let serialized_user = self
            .0
            .get(session_id.to_bytes())
            .with_context(|| format!("failed to retrieve session {session_id}"))?;
        let Some(serialized_user) = serialized_user else { return Ok(None) };
        let session = User::decode(serialized_user.as_ref())
            .with_context(|| format!("failed to deserialize session {session_id}"))?;
        Ok((session.expires_at > Utc::now().timestamp()).then_some(session))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{db::Db, models::new_session_id};

    #[test]
    fn unknown_session_ok() -> Result {
        let session = Db::open_temporary()?
            .session_manager()?
            .get(new_session_id())?;
        assert!(session.is_none());
        Ok(())
    }

    #[test]
    fn known_session_ok() -> Result {
        let manager = Db::open_temporary()?.session_manager()?;
        let session_id = new_session_id();
        manager.insert(
            session_id,
            &User {
                access_token: "test".to_string(),
                expires_at: Utc::now().timestamp() + 10,
                account_id: 0,
                nickname: "test".to_string(),
            },
        )?;
        let user = manager.get(session_id)?;
        assert!(user.is_some());
        Ok(())
    }

    #[test]
    fn expired_session_ok() -> Result {
        let manager = Db::open_temporary()?.session_manager()?;
        let session_id = new_session_id();
        manager.insert(
            session_id,
            &User {
                access_token: "test".to_string(),
                expires_at: Utc::now().timestamp() - 10,
                account_id: 0,
                nickname: "test".to_string(),
            },
        )?;
        let user = manager.get(session_id)?;
        assert!(user.is_none(), "actual user: {user:?}");
        Ok(())
    }
}
