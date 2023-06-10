mod error;
mod extract;
mod partials;
mod prelude;
mod response;
mod session;
mod state;
mod r#static;
mod tracing_;
mod views;

use axum::{routing::get, Router};
use clap::crate_version;
use sentry::integrations::tower::{NewSentryLayer, SentryHttpLayer};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::{cli::WebArgs, prelude::*, web::state::AppState, weegee::WeeGee};

/// Run the web application.
pub async fn run(args: WebArgs) -> Result {
    let db = args.db.open()?;
    let wee_gee = WeeGee::new(&args.wargaming.backend_application_id)?;

    if args.update_tankopedia {
        db.tankopedia_manager()?
            .prepopulate()?
            .update(wee_gee.get_tankopedia().await?)?;
    }

    let app = create_app(AppState::new(
        &db,
        &args.wargaming.frontend_application_id,
        wee_gee,
        &args.public_address,
    )?);
    info!(version = crate_version!(), endpoint = ?args.bind_endpoint, "🚀 running…");
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
        .route("/", get(views::index::get))
        .route("/welcome", get(views::authenticate::get))
        .route("/sign-out", get(views::sign_out::get))
        .route("/profile/:account_id", get(views::profile::get))
        .route("/favicon.ico", get(r#static::get_favicon))
        .route("/apple-touch-icon.png", get(r#static::get_apple_touch_icon))
        .route("/icon-192.png", get(r#static::get_icon_192))
        .route("/icon-512.png", get(r#static::get_icon_512))
        .route("/home.png", get(r#static::get_home_icon))
        .layer(tracing_layer)
        .with_state(state)
}
