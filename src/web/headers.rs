use axum::http::{header, HeaderName};

pub const CACHE_PUBLIC_WEEK: (HeaderName, &str) = (header::CACHE_CONTROL, "max-age=604800, public");
pub const CONTENT_TYPE_MICROSOFT_ICON: (HeaderName, &str) =
    (header::CONTENT_TYPE, "image/vnd.microsoft.icon");
pub const CONTENT_TYPE_PNG: (HeaderName, &str) = (header::CONTENT_TYPE, "image/png");
