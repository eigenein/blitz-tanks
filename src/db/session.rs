//! Database's session manager. Do not confuse with the session cookie manager.

use prost::Message;
use sled::Tree;
use uuid::Uuid;

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
    pub fn insert(&self, session_id: Uuid, user: &User) -> Result {
        self.0
            .insert(session_id.as_bytes(), user.encode_to_vec())
            .with_context(|| format!("failed to insert the session {session_id:?}"))?;
        Ok(())
    }
}
