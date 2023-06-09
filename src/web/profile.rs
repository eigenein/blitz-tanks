use tracing::instrument;

use crate::{
    models::User,
    web::{models::ValidatedAccountId, partials::*, prelude::*},
};

#[instrument(skip_all, fields(account_id = account_id))]
pub async fn get(
    ValidatedAccountId(account_id): ValidatedAccountId,
    user: User,
) -> impl IntoResponse {
    html! {
        (head())
        body {
            section.hero.is-info.is-small {
                div.hero-head {
                    (navbar())
                }

                div.hero-body {
                    div.container {
                        p.title.has-text-weight-medium { (user.nickname) }
                        p.subtitle.has-text-weight-light {
                            "Here you can "
                            span.has-text-weight-medium { "rate" }
                            " vehicles you played and "
                            span.has-text-weight-medium { "discover" }
                            " new ones"
                        }
                    }
                }

                div.hero-foot {
                    nav.tabs.is-medium.is-boxed {
                        div.container {
                            ul {
                                li {
                                    a {
                                        span.icon { i.fa-solid.fa-star-half-stroke aria-hidden="true" {} }
                                        span { "Your vehicles" }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            (footer())
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

    use crate::{
        prelude::Result,
        web::{authenticate::Session, create_app, state::AppState},
    };

    #[tokio::test]
    async fn own_profile_ok() -> Result {
        let state = AppState::new_test()?;
        let session_id = state.db.session_manager()?.insert_test_session()?;
        let request = Request::builder()
            .uri("/profile/1")
            .header("Cookie", format!("{}={session_id}", Session::SESSION_COOKIE_NAME))
            .body(Body::empty())?;
        let response = create_app(state).oneshot(request).await?;
        assert_eq!(response.status(), StatusCode::OK);
        Ok(())
    }

    #[tokio::test]
    async fn reject_anonymous_profile_ok() -> Result {
        let app = create_app(AppState::new_test()?);
        let request = Request::builder().uri("/profile/1").body(Body::empty())?;
        let response = app.oneshot(request).await?;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
        Ok(())
    }
}
