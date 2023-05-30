mod prelude;

use std::time::Duration;

use axum::body::Body;
use axum::http::{Request, Response};
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
        .route("/icon-192.png", get(get_icon_192))
        .route("/icon-512.png", get(get_icon_512))
        .layer(layer);
    axum::Server::bind(&args.bind_endpoint)
        .serve(app.into_make_service())
        .await
        .context("the server has failed")
}

fn on_request(request: &Request<Body>, span: &Span) {
    info!(parent: span, method = ?request.method(), path = request.uri().path(), "🛫 started");
}

fn on_response<B>(response: &Response<B>, latency: Duration, span: &Span) {
    if response.status().is_server_error() {
        error!(parent: span, status = ?response.status(), ?latency, "🛬 failed");
    } else if response.status().is_client_error() {
        warn!(parent: span, status = ?response.status(), ?latency, "🛬 finished");
    } else {
        info!(parent: span, status = ?response.status(), ?latency, "🛬 finished");
    }
}

#[allow(clippy::needless_pass_by_value)]
fn on_failure(_error: ServerErrorsFailureClass, _latency: Duration, span: &Span) {
    error!(parent: span, "❌ something went wrong");
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
                            div.column.is-half-widescreen.is-offset-one-quarter-widescreen {
                                p.title.has-text-weight-light {
                                    "Rate "
                                    span.has-text-weight-medium { "World of Tanks Blitz" }
                                    " vehicles"
                                }
                                p.subtitle.has-text-weight-light {
                                    "Get "
                                    span.has-text-weight-medium { "personal" }
                                    " recommendations"
                                }
                                form {
                                    div.field {
                                        div.control {
                                            a.button.is-warning.is-large.is-responsive {
                                                span.icon { i.fa-solid.fa-right-to-bracket {} }
                                                strong { "Sign in" }
                                            }
                                        }
                                        p.help {
                                            "with your " a href="https://wargaming.net/personal/" { "Wargaming.net ID" }
                                        }
                                    }
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

async fn get_favicon() -> impl IntoResponse {
    (
        [
            Headers::CONTENT_TYPE_MICROSOFT_ICON,
            Headers::CACHE_PUBLIC_WEEK,
        ],
        include_bytes!("web/favicon.ico"),
    )
}

async fn get_apple_touch_icon() -> impl IntoResponse {
    (
        [Headers::CONTENT_TYPE_PNG, Headers::CACHE_PUBLIC_WEEK],
        include_bytes!("web/apple-touch-icon.png"),
    )
}

async fn get_icon_192() -> impl IntoResponse {
    (
        [Headers::CONTENT_TYPE_PNG, Headers::CACHE_PUBLIC_WEEK],
        include_bytes!("web/icon-192.png"),
    )
}

async fn get_icon_512() -> impl IntoResponse {
    (
        [Headers::CONTENT_TYPE_PNG, Headers::CACHE_PUBLIC_WEEK],
        include_bytes!("web/icon-512.png"),
    )
}
