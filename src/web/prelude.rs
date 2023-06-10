pub use axum::response::IntoResponse;
pub use maud::{html, Markup, PreEscaped, Render, DOCTYPE};

pub use crate::web::error::WebError;

pub type WebResult<T> = Result<T, WebError>;
