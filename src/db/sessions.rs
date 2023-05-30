use prost::Message;

#[derive(Clone)]
pub struct Sessions(sled::Tree);

impl From<sled::Tree> for Sessions {
    fn from(tree: sled::Tree) -> Self {
        Self(tree)
    }
}

#[derive(Message)]
pub struct User {}
