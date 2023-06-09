//! Index view.

use axum::extract::State;
use maud::{html, Markup};
use tracing::instrument;

use crate::web::{authenticate::Session, partials::*, state::*};

/// Index route handler.
#[instrument(skip_all)]
pub async fn get(State(state): State<AppState>, session: Session) -> Markup {
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
                                            a.button.is-warning.is-large.is-responsive."px-6" href=(state.sign_in_url) {
                                                span.icon { i.fa-solid.fa-right-to-bracket {} }
                                                strong { "Sign in" }
                                            }
                                        }
                                        p.help {
                                            "with your "
                                            a href="https://wargaming.net/personal/" { "Wargaming.net ID" }
                                        }
                                    }
                                }

                                article.message.is-warning."mt-6" {
                                    div.message-header {
                                        "Work in progress"
                                    }
                                    div.message-body {
                                        "Some functionality may be unimplemented or broken."
                                        " Only European region is currently supported"
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

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    use super::*;
    use crate::{prelude::Result, web::create_app};

    #[tokio::test]
    async fn index_ok() -> Result {
        let app = create_app(AppState::new_test()?);
        let request = Request::builder().uri("/").body(Body::empty())?;
        let response = app.oneshot(request).await?;
        assert_eq!(response.status(), StatusCode::OK);
        Ok(())
    }
}
