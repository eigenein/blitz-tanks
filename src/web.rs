mod error;
mod index;
mod partials;
mod prelude;
mod session;
mod state;
mod r#static;
mod tracing_;

use axum::{routing::get, Router};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use crate::{cli::WebArgs, db::Db, prelude::*, web::state::SignInUrl};

/// Run the web application.
#[instrument(skip_all)]
pub async fn run(args: WebArgs) -> Result {
    info!(endpoint = ?args.bind_endpoint, "🚀 running…");
    let app = create_app(
        args.db.open()?,
        SignInUrl::new(&args.wargaming.application_id, &args.domain_name),
    );
    axum::Server::bind(&args.bind_endpoint)
        .serve(app.into_make_service())
        .await
        .context("the web application has failed")
}

pub fn create_app(db: Db, sign_in_url: SignInUrl) -> Router {
    let tracing_layer = ServiceBuilder::new().layer(
        TraceLayer::new_for_http()
            .on_request(tracing_::on_request)
            .on_response(tracing_::on_response)
            .on_failure(tracing_::on_failure),
    );
    Router::new()
        .route("/", get(index::get))
        .route("/favicon.ico", get(r#static::get_favicon))
        .route("/apple-touch-icon.png", get(r#static::get_apple_touch_icon))
        .route("/icon-192.png", get(r#static::get_icon_192))
        .route("/icon-512.png", get(r#static::get_icon_512))
        .layer(tracing_layer)
        .with_state(sign_in_url)
        .with_state(db)
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    use super::*;

    #[tokio::test]
    async fn test_index() -> Result {
        let app = create_app(Db::open_temporary()?, SignInUrl::new_test());
        let request = Request::builder().uri("/").body(Body::empty())?;
        let response = app.oneshot(request).await?;
        assert_eq!(response.status(), StatusCode::OK);
        Ok(())
    }
}
