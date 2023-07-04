use std::convert::Infallible;

use axum::{
    extract::rejection::PathRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sentry::capture_error;
use tracing::{error, warn};

use crate::{prelude::*, wg::WgError};

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

    #[error("unauthorized")]
    Unauthorized,

    #[error("forbidden: {0:?}")]
    Forbidden(ForbiddenReason),

    #[error("not found")]
    ImATeapot,

    #[error("service unavailable")]
    ServiceUnavailable(#[source] Error),
}

#[derive(Debug)]
pub enum ForbiddenReason {
    NonExistentAccount,
    NoPrivateAccess,
    NonOwner,
    InvalidToken,
}

impl From<Infallible> for WebError {
    #[inline]
    fn from(_: Infallible) -> Self {
        unreachable!("infallible error")
    }
}

impl From<PathRejection> for WebError {
    #[inline]
    fn from(error: PathRejection) -> Self {
        Self::BadRequest(anyhow!("path rejected: {:#}", error))
    }
}

impl From<WgError> for WebError {
    #[inline]
    fn from(error: WgError) -> Self {
        match error {
            WgError::InvalidToken { .. } => Self::Forbidden(ForbiddenReason::InvalidToken),
            WgError::Api { code, message } => Self::ServiceUnavailable(anyhow!(
                "Wargaming.net API is not available: {code}/{message}"
            )),
            WgError::Request(error) => Self::InternalServerError(error),
        }
    }
}

impl IntoResponse for WebError {
    fn into_response(self) -> Response {
        capture_error(&self);

        let status_code = match &self {
            Self::BadRequest(error) => {
                warn!("❌ Bad request: {error:#}");
                StatusCode::BAD_REQUEST
            }

            Self::Forbidden(reason) => {
                error!("❌ Forbidden: {reason:?}");
                StatusCode::FORBIDDEN
            }

            Self::InternalServerError(error) => {
                error!("💥 Internal server error: {error:#}");
                StatusCode::INTERNAL_SERVER_ERROR
            }

            Self::ServiceUnavailable(error) => {
                error!("📴 Service unavailable: {error:#}");
                StatusCode::SERVICE_UNAVAILABLE
            }

            Self::Unauthorized => {
                error!("❌ Unauthorized");
                StatusCode::UNAUTHORIZED
            }

            Self::ImATeapot => {
                error!("🫖 I'm a teapot");
                StatusCode::IM_A_TEAPOT
            }
        };
        status_code.into_response()
    }
}
