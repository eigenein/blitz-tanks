use prost::Message;
use sled::Tree;

/// Wrapper around the tree to manage client-side sessions.
#[derive(Clone)]
pub struct SessionManager(Tree);

impl From<Tree> for SessionManager {
    fn from(tree: Tree) -> Self {
        Self(tree)
    }
}

/// Authenticated user, which is associated with the session.
#[derive(Message)]
pub struct User {}
