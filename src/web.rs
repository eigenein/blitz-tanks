mod error;
mod index;
mod partials;
mod prelude;
mod session;
mod state;
mod r#static;
mod tracing_;

use axum::routing::get;
use axum::Router;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use crate::cli::WebArgs;
use crate::prelude::*;
use crate::web::state::SignInUrl;

/// Run the web application.
#[instrument(skip_all)]
pub async fn run(args: WebArgs) -> Result {
    info!(endpoint = ?args.bind_endpoint, "🚀 running…");
    let layer = ServiceBuilder::new().layer(
        TraceLayer::new_for_http()
            .on_request(tracing_::on_request)
            .on_response(tracing_::on_response)
            .on_failure(tracing_::on_failure),
    );
    let app = Router::new()
        .route("/", get(index::get))
        .route("/favicon.ico", get(r#static::get_favicon))
        .route("/apple-touch-icon.png", get(r#static::get_apple_touch_icon))
        .route("/icon-192.png", get(r#static::get_icon_192))
        .route("/icon-512.png", get(r#static::get_icon_512))
        .layer(layer)
        .with_state(SignInUrl::new(&args.wargaming.application_id, &args.domain_name))
        .with_state(args.db.open()?);
    axum::Server::bind(&args.bind_endpoint)
        .serve(app.into_make_service())
        .await
        .context("the web application has failed")
}
