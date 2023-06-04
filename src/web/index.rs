//! Index view.

use axum::extract::State;

use crate::web::{partials::*, prelude::*, state::*};

/// Handle the GET index request.
pub async fn get(State(SignInUrl(sign_in_url)): State<SignInUrl>) -> Markup {
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
                                            a.button.is-warning.is-large.is-responsive."px-6" href=(sign_in_url) {
                                                span.icon { i.fa-solid.fa-right-to-bracket {} }
                                                strong { "Sign in" }
                                            }
                                        }
                                        p.help {
                                            "with your "
                                            a href="https://wargaming.net/personal/" { "Wargaming.net ID" }
                                            " (only Europe)"
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
