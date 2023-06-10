mod authenticate;
mod error;
mod extract;
mod headers;
mod index;
mod partials;
mod prelude;
mod profile;
mod response;
mod state;
mod r#static;
mod tracing_;

use axum::{routing::get, Router};
use clap::crate_version;
use sentry::integrations::tower::{NewSentryLayer, SentryHttpLayer};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::{info, instrument};

use crate::{cli::WebArgs, prelude::*, web::state::AppState};

/// Run the web application.
#[instrument(skip_all, err)]
pub async fn run(args: WebArgs) -> Result {
    info!(version = crate_version!(), endpoint = ?args.bind_endpoint, "🚀 running…");
    let app = create_app(AppState::new(
        args.db.open()?,
        &args.wargaming.frontend_application_id,
        &args.wargaming.backend_application_id,
        &args.public_address,
    )?);
    axum::Server::bind(&args.bind_endpoint)
        .serve(app.into_make_service())
        .await
        .context("the web application has failed")
}

pub fn create_app(state: AppState) -> Router {
    let tracing_layer = ServiceBuilder::new()
        .layer(
            TraceLayer::new_for_http()
                .on_request(tracing_::on_request)
                .on_response(tracing_::on_response)
                .on_failure(tracing_::on_failure),
        )
        .layer(SentryHttpLayer::with_transaction())
        .layer(NewSentryLayer::new_from_top());
    Router::new()
        .route("/", get(index::get))
        .route("/welcome", get(authenticate::get))
        .route("/profile/:account_id", get(profile::get))
        .route("/favicon.ico", get(r#static::get_favicon))
        .route("/apple-touch-icon.png", get(r#static::get_apple_touch_icon))
        .route("/icon-192.png", get(r#static::get_icon_192))
        .route("/icon-512.png", get(r#static::get_icon_512))
        .route("/home.png", get(r#static::get_home_icon))
        .layer(tracing_layer)
        .with_state(state)
}
