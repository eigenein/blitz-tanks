pub type Result<T = ()> = anyhow::Result<T>;
pub use anyhow::Context;
pub use tracing::{debug, error, info, instrument, warn, Level, Span};
