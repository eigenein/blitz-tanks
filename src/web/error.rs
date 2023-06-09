use std::convert::Infallible;

use axum::{
    extract::rejection::PathRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sentry::capture_error;
use tracing::{error, warn};

use crate::prelude::*;

/// Custom error enumeration, which can be used in the web handlers.
///
/// Axum makes it **really** difficult to implement proper error handling
/// for the request handlers with custom tracing and Sentry integration, hence this workaround.
#[derive(thiserror::Error, Debug)]
pub enum WebError {
    /// Any uncaught Anyhow error is by default an internal server error.
    #[error("internal server error")]
    InternalServerError(#[from] Error),

    #[error("bad request")]
    BadRequest(#[source] Error),

    #[error("forbidden")]
    Forbidden,

    #[error("service unavailable: #{code} {message}")]
    ServiceUnavailable { code: u16, message: String },
}

impl From<Infallible> for WebError {
    fn from(_: Infallible) -> Self {
        unreachable!("infallible error")
    }
}

impl From<PathRejection> for WebError {
    fn from(error: PathRejection) -> Self {
        Self::BadRequest(anyhow!("path rejected: {:#}", error))
    }
}

impl IntoResponse for WebError {
    fn into_response(self) -> Response {
        let status_code = match &self {
            Self::BadRequest(error) => {
                warn!("❌ bad request: {error:#}");
                StatusCode::BAD_REQUEST
            }

            Self::Forbidden => StatusCode::FORBIDDEN,

            Self::InternalServerError(error) => {
                error!("💥 internal server error: {error:#}");
                StatusCode::INTERNAL_SERVER_ERROR
            }

            Self::ServiceUnavailable { code, message } => {
                error!("📴 service unavailable: #{code} {message}");
                capture_error(&self);
                StatusCode::SERVICE_UNAVAILABLE
            }
        };
        status_code.into_response()
    }
}
