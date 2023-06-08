use axum::{http::StatusCode, response::Response};
use sentry::integrations::anyhow::capture_anyhow;
use tracing::error;

use crate::web::prelude::*;

/// Wrapper around [`anyhow::Error`], which can be used in the web handlers.
///
/// Axum makes it **really** difficult to implement proper error handling
/// for request handlers with tracing and Sentry integration, hence this workaround.
pub struct InternalServerError(anyhow::Error);

impl<E: Into<anyhow::Error>> From<E> for InternalServerError {
    fn from(error: E) -> Self {
        Self(error.into())
    }
}

impl IntoResponse for InternalServerError {
    fn into_response(self) -> Response {
        capture_anyhow(&self.0);
        error!("{:#}", self.0);
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}
