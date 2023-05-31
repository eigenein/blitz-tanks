use axum::{http::StatusCode, response::Response};

use crate::web::prelude::*;

/// Wrapper around [`anyhow::Error`], which can be used in the web handlers.
pub struct WebError(anyhow::Error);

impl<E: Into<anyhow::Error>> From<E> for WebError {
    fn from(error: E) -> Self {
        Self(error.into())
    }
}

impl IntoResponse for WebError {
    fn into_response(self) -> Response {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

/// Convenience alias for web handler's result types.
pub type WebResult<T = ()> = Result<T, WebError>;
