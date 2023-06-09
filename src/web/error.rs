use axum::{
    extract::rejection::PathRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::{error, warn};

/// Custom error enumeration, which can be used in the web handlers.
///
/// Axum makes it **really** difficult to implement proper error handling
/// for the request handlers with custom tracing and Sentry integration, hence this workaround.
#[derive(thiserror::Error, Debug)]
pub enum WebError {
    /// Any uncaught Anyhow error is an internal server error.
    #[error("internal server error")]
    InternalServerError(#[from] anyhow::Error),

    #[error("forbidden")]
    Forbidden,

    /// Infallible variant, it only exists to convert from Axum's «infallible errors».
    #[error("infallible")]
    Infallible(#[from] std::convert::Infallible),

    #[error("invalid path: `{0}`")]
    PathRejection(#[from] PathRejection),
}

impl IntoResponse for WebError {
    fn into_response(self) -> Response {
        let status_code = match self {
            Self::Infallible(_) => unreachable!("infallible error"),

            Self::Forbidden => StatusCode::FORBIDDEN,

            Self::PathRejection(reason) => {
                warn!("❌ path rejected: {reason}");
                StatusCode::BAD_REQUEST
            }

            Self::InternalServerError(error) => {
                error!("💥 internal server error: {error:#}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };
        status_code.into_response()
    }
}
