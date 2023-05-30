use crate::web::prelude::*;

pub async fn get_favicon() -> impl IntoResponse {
    (
        [
            Headers::CONTENT_TYPE_MICROSOFT_ICON,
            Headers::CACHE_PUBLIC_WEEK,
        ],
        include_bytes!("favicon.ico"),
    )
}

pub async fn get_apple_touch_icon() -> impl IntoResponse {
    (
        [Headers::CONTENT_TYPE_PNG, Headers::CACHE_PUBLIC_WEEK],
        include_bytes!("apple-touch-icon.png"),
    )
}

pub async fn get_icon_192() -> impl IntoResponse {
    (
        [Headers::CONTENT_TYPE_PNG, Headers::CACHE_PUBLIC_WEEK],
        include_bytes!("icon-192.png"),
    )
}

pub async fn get_icon_512() -> impl IntoResponse {
    (
        [Headers::CONTENT_TYPE_PNG, Headers::CACHE_PUBLIC_WEEK],
        include_bytes!("icon-512.png"),
    )
}
