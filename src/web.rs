mod prelude;

use std::time::Duration;

use axum::body::Body;
use axum::http::{header, Request, Response};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use tower::ServiceBuilder;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::trace::TraceLayer;

use self::prelude::*;
use crate::cli::WebArgs;
use crate::prelude::*;

#[instrument(skip_all)]
pub async fn run(args: WebArgs) -> Result {
    info!(endpoint = ?args.bind_endpoint, "🚀 running…");
    let layer = ServiceBuilder::new().layer(
        TraceLayer::new_for_http()
            .on_request(on_request)
            .on_response(on_response)
            .on_failure(on_failure),
    );
    let app = Router::new()
        .route("/", get(get_index))
        .route("/favicon.ico", get(get_favicon))
        .route("/apple-touch-icon.png", get(get_apple_touch_icon))
        .layer(layer);
    axum::Server::bind(&args.bind_endpoint)
        .serve(app.into_make_service())
        .await
        .context("the server has failed")
}

async fn get_favicon() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "image/vnd.microsoft.icon")],
        include_bytes!("web/favicon.ico"),
    )
}

async fn get_apple_touch_icon() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "image/png")],
        include_bytes!("web/apple-touch-icon.png"),
    )
}

async fn get_index() -> Markup {
    html! {
        (head())
        body {
            section.hero.is-fullheight {
                div.hero-head {
                    (navbar())
                }

                div.hero-body {
                    div.container {
                        div.columns {
                            div.column."is-6"."is-offset-3" {
                                p.title { "Vehicle recommender system" }
                                p.subtitle {
                                    "It's like for movies, yet for World of Tanks Blitz!"
                                }
                            }
                        }
                    }
                }
            }

            ((footer()))
        }
    }
}

fn on_request(request: &Request<Body>, span: &Span) {
    info!(parent: span, method = ?request.method(), path = request.uri().path(), "started");
}

fn on_response<B>(response: &Response<B>, latency: Duration, span: &Span) {
    info!(parent: span, status = ?response.status(), ?latency);
}

#[allow(clippy::needless_pass_by_value)]
fn on_failure(_error: ServerErrorsFailureClass, _latency: Duration, span: &Span) {
    error!(parent: span, "something went wrong");
}
