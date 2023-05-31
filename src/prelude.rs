pub type Result<T = ()> = anyhow::Result<T>;
pub use ::anyhow::{Context, Error};
pub use ::tracing::{debug, error, info, instrument, warn, Level, Span};
