//! Index view.

use axum::{extract::State, response::Redirect};
use tracing::{info, instrument};

use crate::{
    models::User,
    web::{prelude::*, response::OptionalRedirect, session::Session, state::*, views::partials::*},
};

/// Index route handler.
#[instrument(skip_all)]
pub async fn get(State(state): State<AppState>, session: Session) -> OptionalRedirect {
    if let Session::Authenticated(User { account_id, .. }) = session {
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

                                article.message.is-warning."mt-6" {
                                    div.message-header {
                                        "Work in progress"
                                    }
                                    div.message-body.content {
                                        div.content {
                                            p { "Disclaimer:" }
                                            ul {
                                                li { "Only core vital features are implemented" }
                                                li { "Some functionality may be unimplemented or broken" }
                                                li { "Only European region is currently supported" }
                                            }
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
    use crate::{prelude::Result, web::create_app};

    #[tokio::test]
    async fn index_ok() -> Result {
        let app = create_app(AppState::new_test()?);
        let request = Request::builder().uri("/").body(Body::empty())?;
        let response = app.oneshot(request).await?;
        assert_eq!(response.status(), StatusCode::OK);
        Ok(())
    }

    #[tokio::test]
    async fn redirect_ok() -> Result {
        let state = AppState::new_test()?;
        let session_id = state.session_manager.insert_test_session()?;
        let request = Request::builder()
            .uri("/")
            .header("Cookie", format!("{}={session_id}", Session::SESSION_COOKIE_NAME))
            .body(Body::empty())?;
        let response = create_app(state).oneshot(request).await?;
        assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);
        let headers = response.headers();
        assert_eq!(headers.get("Location").unwrap(), "/profile/0");
        Ok(())
    }
}
