use mongodb::Collection;

/// Collaborative filtering model repository.
#[derive(Clone)]
pub struct Models(Collection<()>);
