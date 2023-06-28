mod error;
mod prelude;
mod response;
mod result;
mod state;
mod r#static;
mod tracing_;
mod user;
mod views;

use std::{net::SocketAddr, sync::Arc, time::Duration};

use arc_swap::ArcSwap;
use axum::{
    routing::{get, post},
    Router, Server,
};
use clap::{crate_version, Args};
use futures::future::try_join;
use sentry::integrations::tower::{NewSentryLayer, SentryHttpLayer};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::{
    cli::DbArgs, db::models::Models, prelude::*, trainer::item_item::Model, web::state::AppState,
    wg::Wg,
};

#[derive(Args)]
pub struct Web {
    /// Web application bind endpoint.
    #[clap(
        long,
        env = "BLITZ_TANKS_BIND_ENDPOINT",
        default_value = "127.0.0.1:8080"
    )]
    bind_endpoint: SocketAddr,

    /// Wargaming.net application ID for the front-end app.
    #[clap(long = "frontend-app-id", env = "BLITZ_TANKS_FRONTEND_APPLICATION_ID")]
    frontend_application_id: String,

    /// Wargaming.net application ID for the back-end app.
    #[clap(long = "backend-app-id", env = "BLITZ_TANKS_BACKEND_APPLICATION_ID")]
    backend_application_id: String,

    /// Public address used in the hyperlinks.
    #[clap(
        long,
        env = "BLITZ_TANKS_PUBLIC_ADDRESS",
        default_value = "localhost:8080"
    )]
    public_address: String,

    #[clap(flatten)]
    db: DbArgs,

    /// Update the tankopedia database on startup.
    #[clap(long, env = "BLITZ_TANKS_UPDATE_TANKOPEDIA")]
    update_tankopedia: bool,

    /// Model reload interval, in seconds.
    #[clap(
        long,
        default_value = "3600",
        env = "BLITZ_TANKS_MODEL_UPDATE_INTERVAL"
    )]
    model_update_interval_secs: u64,
}

impl Web {
    /// Run the web application.
    pub async fn run(self) -> Result {
        let db = self.db.open().await?;
        let wg = Wg::new(&self.backend_application_id)?;

        if self.update_tankopedia {
            db.tankopedia()
                .await?
                .prepopulate()
                .await?
                .update(wg.get_tankopedia().await?)
                .await?;
        }

        let state =
            AppState::new(&db, &self.frontend_application_id, wg, &self.public_address).await?;
        let reloader = Self::run_model_reloader(
            state.model.clone(),
            db.models().await?,
            Duration::from_secs(self.model_update_interval_secs),
        );
        let server = self.serve(state);

        try_join(server, reloader).await.context("the application has failed")?;
        Ok(())
    }

    pub fn create_router(state: AppState) -> Router {
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
            .route("/discover", get(views::discover::get))
            .route("/favicon.ico", get(r#static::get_favicon))
            .route("/apple-touch-icon.png", get(r#static::get_apple_touch_icon))
            .route("/icon-192.png", get(r#static::get_icon_192))
            .route("/icon-512.png", get(r#static::get_icon_512))
            .route("/home.png", get(r#static::get_home_icon))
            .route("/bulma-patches.css/:version", get(r#static::get_bulma_patches))
            .layer(tracing_layer)
            .with_state(state)
    }

    async fn serve(self, state: AppState) -> Result {
        info!(version = crate_version!(), endpoint = ?self.bind_endpoint, "🚀 Running the web server…");
        Server::bind(&self.bind_endpoint)
            .serve(Self::create_router(state).into_make_service())
            .await
            .context("the web server has failed")
    }

    /// Runs the model reloader forever.
    async fn run_model_reloader(
        model: Arc<ArcSwap<Model>>,
        models: Models,
        interval: Duration,
    ) -> Result {
        info!(?interval, "🚀 Running the model updater…");
        loop {
            tokio::time::sleep(interval).await;
            info!("⏰ Reloading the model…");
            let new_model = models.get_latest().await?;
            model.store(Arc::new(new_model));
        }
    }
}
