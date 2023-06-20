mod error;
mod extract;
mod prelude;
mod response;
mod state;
mod r#static;
mod tracing_;
mod views;

use std::net::SocketAddr;

use axum::{
    routing::{get, post},
    Router,
};
use clap::{crate_version, Args};
use sentry::integrations::tower::{NewSentryLayer, SentryHttpLayer};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::{cli::DbArgs, prelude::*, web::state::AppState, wg::Wg};

#[derive(Args)]
pub struct WebArgs {
    /// Web application bind endpoint.
    #[clap(
        long,
        env = "BLITZ_TANKS_BIND_ENDPOINT",
        default_value = "127.0.0.1:8080"
    )]
    pub bind_endpoint: SocketAddr,

    #[clap(flatten)]
    pub wargaming: WargamingArgs,

    /// Public address used in the hyperlinks.
    #[clap(
        long,
        env = "BLITZ_TANKS_PUBLIC_ADDRESS",
        default_value = "localhost:8080"
    )]
    pub public_address: String,

    #[clap(flatten)]
    pub db: DbArgs,

    /// Update the tankopedia database on startup.
    #[clap(long, env = "BLITZ_TANKS_UPDATE_TANKOPEDIA")]
    pub update_tankopedia: bool,
}

#[derive(Args)]
pub struct WargamingArgs {
    /// Wargaming.net application ID for the front-end app.
    #[clap(long = "frontend-app-id", env = "BLITZ_TANKS_FRONTEND_APPLICATION_ID")]
    pub frontend_application_id: String,

    /// Wargaming.net application ID for the back-end app.
    #[clap(long = "backend-app-id", env = "BLITZ_TANKS_BACKEND_APPLICATION_ID")]
    pub backend_application_id: String,
}

/// Run the web application.
pub async fn run(args: WebArgs) -> Result {
    let db = args.db.open().await?;
    let wee_gee = Wg::new(&args.wargaming.backend_application_id)?;

    if args.update_tankopedia {
        db.tankopedia()
            .await?
            .prepopulate()
            .await?
            .update(wee_gee.get_tankopedia().await?)
            .await?;
    }

    let state =
        AppState::new(&db, &args.wargaming.frontend_application_id, wee_gee, &args.public_address)
            .await?;
    info!(version = crate_version!(), endpoint = ?args.bind_endpoint, "🚀 running…");
    axum::Server::bind(&args.bind_endpoint)
        .serve(create_app(state).into_make_service())
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
        .route("/profile/:account_id/vehicle/:tank_id/like", post(views::profile::like_vehicle))
        .route(
            "/profile/:account_id/vehicle/:tank_id/dislike",
            post(views::profile::dislike_vehicle),
        )
        .route(
            "/profile/:account_id/vehicle/:tank_id/unrate",
            post(views::profile::unrate_vehicle),
        )
        .route("/favicon.ico", get(r#static::get_favicon))
        .route("/apple-touch-icon.png", get(r#static::get_apple_touch_icon))
        .route("/icon-192.png", get(r#static::get_icon_192))
        .route("/icon-512.png", get(r#static::get_icon_512))
        .route("/home.png", get(r#static::get_home_icon))
        .route("/bulma-patches.css/:version", get(r#static::get_bulma_patches))
        .layer(tracing_layer)
        .with_state(state)
}
