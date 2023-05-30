pub use ::axum::response::IntoResponse;
pub use ::maud::{html, Markup, PreEscaped};
use axum::http::{header, HeaderName};
pub use maud::DOCTYPE;

pub use crate::web::error::{WebError, WebResult};

/// Some frequently used headers.
pub struct Headers {}

impl Headers {
    pub const CACHE_PUBLIC_WEEK: (HeaderName, &'static str) =
        (header::CACHE_CONTROL, "max-age=604800, public");
    pub const CONTENT_TYPE_MICROSOFT_ICON: (HeaderName, &'static str) =
        (header::CONTENT_TYPE, "image/vnd.microsoft.icon");
    pub const CONTENT_TYPE_PNG: (HeaderName, &'static str) = (header::CONTENT_TYPE, "image/png");
}
