//! ```text
//! Add this to your HTML <head>:
//!
//!     <link rel="icon" href="/favicon.ico" sizes="any">
//!     <link rel="apple-touch-icon" href="/apple-touch-icon.png">
//!
//! Add this to your app's manifest.json:
//!
//!     ...
//!     {
//!       "icons": [
//!         { "src": "/favicon.ico", "type": "image/x-icon", "sizes": "16x16 32x32" },
//!         { "src": "/icon-192.png", "type": "image/png", "sizes": "192x192" },
//!         { "src": "/icon-512.png", "type": "image/png", "sizes": "512x512" },
//!         { "src": "/icon-192-maskable.png", "type": "image/png", "sizes": "192x192", "purpose": "maskable" },
//!         { "src": "/icon-512-maskable.png", "type": "image/png", "sizes": "512x512", "purpose": "maskable" }
//!       ]
//!     }
//!     ...
//! ```

use axum::response::IntoResponse;

use crate::web::headers::*;

pub async fn get_favicon() -> impl IntoResponse {
    (
        [CONTENT_TYPE_MICROSOFT_ICON, CACHE_PUBLIC_WEEK],
        include_bytes!("static/favicon.ico"),
    )
}

pub async fn get_apple_touch_icon() -> impl IntoResponse {
    (
        [CONTENT_TYPE_PNG, CACHE_PUBLIC_WEEK],
        include_bytes!("static/apple-touch-icon.png"),
    )
}

pub async fn get_icon_192() -> impl IntoResponse {
    ([CONTENT_TYPE_PNG, CACHE_PUBLIC_WEEK], include_bytes!("static/icon-192.png"))
}

pub async fn get_icon_512() -> impl IntoResponse {
    ([CONTENT_TYPE_PNG, CACHE_PUBLIC_WEEK], include_bytes!("static/icon-512.png"))
}

pub async fn get_home_icon() -> impl IntoResponse {
    ([CONTENT_TYPE_PNG, CACHE_PUBLIC_WEEK], include_bytes!("static/home.png"))
}
