use axum::http::{header, HeaderName};

/// Some frequently used headers.
pub struct Headers {}

impl Headers {
    pub const CACHE_PUBLIC_WEEK: (HeaderName, &'static str) =
        (header::CACHE_CONTROL, "max-age=604800, public");
    pub const CONTENT_TYPE_MICROSOFT_ICON: (HeaderName, &'static str) =
        (header::CONTENT_TYPE, "image/vnd.microsoft.icon");
    pub const CONTENT_TYPE_PNG: (HeaderName, &'static str) = (header::CONTENT_TYPE, "image/png");
}
