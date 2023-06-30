//! ```text
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

use axum::{
    extract::Path,
    headers::HeaderName,
    http::header::{CACHE_CONTROL, CONTENT_TYPE},
    response::IntoResponse,
};
use indoc::indoc;

use crate::{
    prelude::instrument,
    tankopedia::vendored::TANKOPEDIA,
    web::{error::WebError, result::WebResult},
};

const CACHE_PUBLIC_WEEK: (HeaderName, &str) = (CACHE_CONTROL, "max-age=604800, public");
const CACHE_PUBLIC_YEAR: (HeaderName, &str) = (CACHE_CONTROL, "max-age=31536000, public");

const CONTENT_TYPE_CSS: (HeaderName, &str) = (CONTENT_TYPE, "text/css");
const CONTENT_TYPE_MICROSOFT_ICON: (HeaderName, &str) = (CONTENT_TYPE, "image/vnd.microsoft.icon");
const CONTENT_TYPE_PNG: (HeaderName, &str) = (CONTENT_TYPE, "image/png");
const CONTENT_TYPE_WEBP: (HeaderName, &str) = (CONTENT_TYPE, "image/webp");

#[inline]
pub async fn get_favicon() -> impl IntoResponse {
    (
        [CONTENT_TYPE_MICROSOFT_ICON, CACHE_PUBLIC_WEEK],
        include_bytes!("static/favicon.ico"),
    )
}

#[inline]
pub async fn get_apple_touch_icon() -> impl IntoResponse {
    (
        [CONTENT_TYPE_PNG, CACHE_PUBLIC_YEAR],
        include_bytes!("static/apple-touch-icon.png"),
    )
}

#[inline]
pub async fn get_icon_192() -> impl IntoResponse {
    ([CONTENT_TYPE_PNG, CACHE_PUBLIC_YEAR], include_bytes!("static/icon-192.png"))
}

#[inline]
pub async fn get_icon_512() -> impl IntoResponse {
    ([CONTENT_TYPE_PNG, CACHE_PUBLIC_YEAR], include_bytes!("static/icon-512.png"))
}

#[inline]
pub async fn get_home_icon() -> impl IntoResponse {
    ([CONTENT_TYPE_PNG, CACHE_PUBLIC_YEAR], include_bytes!("static/home.png"))
}

/// Get the patches for [Bulma][1].
///
/// Unfortunately, [`bulma-prefers-dark`][2] is not maintained any more,
/// and the Bulma developer [ignores][3] the dark mode for years,
/// so, I have to patch some additional colors.
///
/// [1]: https://bulma.io/
/// [2]: https://github.com/jloh/bulma-prefers-dark
/// [3]: https://github.com/jgthms/bulma/issues/2342
#[inline]
pub async fn get_bulma_patches() -> impl IntoResponse {
    const CSS: &str = indoc! { /* language=css */ r#"
        .has-object-fit-cover { object-fit: cover; }
        .has-object-fit-contain { object-fit: contain; }

        @media (prefers-color-scheme: dark) {
            .has-background-success-light { background-color: hsl(141, 53%, 14%) !important; }
            .has-background-danger-light { background-color: hsl(348, 86%, 14%) !important; }
            .has-background-warning-light { background-color: hsl(48, 100%, 14%) !important; }
            .has-background-info-light { background-color: hsl(204, 71%, 14%) !important; }
        }
    "#};
    ([CONTENT_TYPE_CSS, CACHE_PUBLIC_YEAR], CSS)
}

#[instrument(skip_all, fields(tank_id = tank_id))]
pub async fn get_vehicle_icon(Path(tank_id): Path<u16>) -> WebResult<impl IntoResponse> {
    let Some(vehicle) = &TANKOPEDIA.get(&tank_id) else {
        return Err(WebError::ImATeapot);
    };
    Ok(([CONTENT_TYPE_WEBP, CACHE_PUBLIC_YEAR], vehicle.image_content))
}
