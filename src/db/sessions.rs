use prost::Message;

/// Wrapper around the tree to manage client-side sessions.
#[derive(Clone)]
pub struct Sessions(sled::Tree);

impl From<sled::Tree> for Sessions {
    fn from(tree: sled::Tree) -> Self {
        Self(tree)
    }
}

/// Authenticated user, which is associated with the session.
#[derive(Message)]
pub struct User {}
