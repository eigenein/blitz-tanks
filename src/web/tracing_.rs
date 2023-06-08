//! Tracing handlers for the Axum web application.

use std::time::Duration;

use axum::{
    body::Body,
    http::{Request, Response},
};
use tower_http::classify::ServerErrorsFailureClass;
use tracing::{error, info, warn, Span};

pub fn on_request(request: &Request<Body>, span: &Span) {
    info!(parent: span, method = ?request.method(), path = request.uri().path(), "🛫 started");
}

pub fn on_response<B>(response: &Response<B>, latency: Duration, span: &Span) {
    if response.status().is_server_error() {
        error!(parent: span, status = ?response.status(), ?latency, "🛬 failed");
    } else if response.status().is_client_error() {
        warn!(parent: span, status = ?response.status(), ?latency, "🛬 finished");
    } else {
        info!(parent: span, status = ?response.status(), ?latency, "🛬 finished");
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn on_failure(_error: ServerErrorsFailureClass, latency: Duration, span: &Span) {
    error!(parent: span, ?latency, "💥 something went wrong");
}
