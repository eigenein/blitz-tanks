mod authenticate;
mod error;
mod headers;
mod index;
mod models;
mod partials;
mod profile;
mod state;
mod r#static;
mod tracing_;

use axum::{routing::get, Router};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::{info, instrument};

use crate::{cli::WebArgs, prelude::*, web::state::AppState};

/// Run the web application.
#[instrument(skip_all)]
pub async fn run(args: WebArgs) -> Result {
    info!(endpoint = ?args.bind_endpoint, "🚀 running…");
    let app = create_app(AppState::new(
        args.db.open()?,
        &args.wargaming.application_id,
        &args.domain_name,
    ));
    axum::Server::bind(&args.bind_endpoint)
        .serve(app.into_make_service())
        .await
        .context("the web application has failed")
}

pub fn create_app(state: AppState) -> Router {
    let tracing_layer = ServiceBuilder::new().layer(
        TraceLayer::new_for_http()
            .on_request(tracing_::on_request)
            .on_response(tracing_::on_response)
            .on_failure(tracing_::on_failure),
    );
    Router::new()
        .route("/", get(index::get))
        .route("/authenticate", get(authenticate::get))
        .route("/profile/:account_id", get(profile::get))
        .route("/favicon.ico", get(r#static::get_favicon))
        .route("/apple-touch-icon.png", get(r#static::get_apple_touch_icon))
        .route("/icon-192.png", get(r#static::get_icon_192))
        .route("/icon-512.png", get(r#static::get_icon_512))
        .layer(tracing_layer)
        .with_state(state)
}
