//! Index view.

use axum::{extract::State, response::Redirect};
use either::Either;
use tracing::{info, instrument};

use crate::{
    models::{Anonymous, User},
    web::{prelude::*, response::OptionalRedirect, state::*, views::partials::*},
};

/// Index route handler.
#[instrument(skip_all)]
pub async fn get(
    State(state): State<AppState>,
    session: Either<User, Anonymous>,
) -> OptionalRedirect {
    if let Either::Left(User { account_id, .. }) = session {
        info!(account_id, "👋 welcome");
        return OptionalRedirect::Redirect(Redirect::temporary(&format!("/profile/{account_id}")));
    }

    let markup = html! {
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
                            }
                        }
                    }
                }
            }

            ((footer()))
        }
    };
    OptionalRedirect::Markup(markup)
}

/// Landing's navigation bar.
fn navbar() -> Markup {
    html! {
        nav.navbar role="navigation" aria-label="main navigation" {
            div.container {
                (navbar_brand())
                #navbar.navbar-menu { div.navbar-end {} }
            }
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
    use crate::{prelude::Result, web::Web};

    #[tokio::test]
    #[ignore]
    async fn index_ok() -> Result {
        let app = Web::create_app(AppState::new_test().await?);
        let request = Request::builder().uri("/").body(Body::empty())?;
        let response = app.oneshot(request).await?;
        assert_eq!(response.status(), StatusCode::OK);
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn redirect_ok() -> Result {
        let state = AppState::new_test().await?;
        let session_id = state.session_manager.insert_test_session().await?;
        let request = Request::builder()
            .uri("/")
            .header("Cookie", format!("{}={session_id}", User::SESSION_COOKIE_NAME))
            .body(Body::empty())?;
        let response = Web::create_app(state).oneshot(request).await?;
        assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);
        let headers = response.headers();
        assert_eq!(headers.get("Location").unwrap(), "/profile/0");
        Ok(())
    }
}
