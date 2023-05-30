use axum::http::StatusCode;
use axum::response::Response;

use crate::web::prelude::*;

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

pub type WebResult<T = ()> = Result<T, WebError>;
