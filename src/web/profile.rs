use axum::extract::State;
use tracing::instrument;

use crate::{
    models::User,
    web::{extract::ValidatedAccountId, partials::*, prelude::*, state::AppState},
};

#[instrument(skip_all, fields(account_id = user.account_id))]
pub async fn get(
    _: ValidatedAccountId,
    user: User,
    State(state): State<AppState>,
) -> WebResult<impl IntoResponse> {
    let vehicles_stats = state.vehicle_stats_getter.get(user.account_id).await?;

    let markup = html! {
        (head())
        body {
            (navbar(&user))

            section.section {
                div.container {
                    div.columns.is-multiline.is-tablet {
                        @for stats in vehicles_stats.values() {
                            @let description = state.tankopedia.get(&stats.tank_id);

                            div.column."is-6-tablet"."is-4-desktop"."is-3-widescreen" {
                                div.card {
                                    header.card-header {
                                        p.card-header-title {
                                            @match description {
                                                Some(description) => { (description.name) },
                                                None => { "#" (stats.tank_id) },
                                            }
                                        }
                                    }

                                    div.card-image {
                                        figure.image {
                                            @let url = description
                                                .and_then(|d| d.images.normal_url.as_ref())
                                                .map_or("https://dummyimage.com/1060x774", |url| url.as_str());
                                            img src=(url) loading="lazy";
                                        }
                                    }

                                    footer.card-footer {
                                        a.card-footer-item title="Hate it!" { span.icon.has-text-danger { i.fa-solid.fa-heart-crack {} } }
                                        a.card-footer-item title="Dislike it" { span.icon.has-text-warning { i.fa-solid.fa-thumbs-down {} } }
                                        a.card-footer-item title="Tentative" { span.icon.has-text-info { i.fa-regular.fa-face-meh {} } }
                                        a.card-footer-item title="Like it" { span.icon.has-text-primary { i.fa-solid.fa-thumbs-up {} } }
                                        a.card-footer-item title="Love it!" { span.icon.has-text-success { i.fa-solid.fa-heart {} } }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            (footer())
        }
    };

    Ok(markup)
}

/// Profile navigation bar.
fn navbar(User { account_id, nickname, .. }: &User) -> Markup {
    html! {
        nav.navbar.is-warning role="navigation" aria-label="main navigation" {
            div.container {
                (navbar_brand())

                #navbar.navbar-menu {
                    div.navbar-start {
                        div.navbar-item {
                            span.icon { i.fa-regular.fa-user {} }
                            span { (nickname) }
                        }

                        a.navbar-item href=(format!("/profile/{account_id}")) {
                            span.icon { i.fa-solid.fa-star-half-stroke aria-hidden="true" {} }
                            span { "Rate" }
                        }

                        a.navbar-item href=(format!("/profile/{account_id}/discover")) {
                            span.icon { i.fa-solid.fa-wand-magic-sparkles aria-hidden="true" {} }
                            span { "Discover" }
                        }
                    }
                    div.navbar-end {
                        div.navbar-item {
                            div.field {
                                p.control {
                                    a.button.is-rounded.is-danger {
                                        span.icon { i.fa-solid.fa-right-from-bracket {} }
                                        span { "Sign out" }
                                    }
                                }
                            }
                        }
                    }
                }
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

    use crate::{
        prelude::Result,
        web::{authenticate::Session, create_app, state::AppState},
    };

    #[tokio::test]
    async fn own_profile_ok() -> Result {
        let state = AppState::new_test()?;
        let session_id = state.session_manager.insert_test_session()?;
        let request = Request::builder()
            .uri("/profile/0")
            .header("Cookie", format!("{}={session_id}", Session::SESSION_COOKIE_NAME))
            .body(Body::empty())?;
        let response = create_app(state).oneshot(request).await?;
        assert_eq!(response.status(), StatusCode::OK);
        Ok(())
    }

    #[tokio::test]
    async fn reject_anonymous_profile_ok() -> Result {
        let app = create_app(AppState::new_test()?);
        let request = Request::builder().uri("/profile/0").body(Body::empty())?;
        let response = app.oneshot(request).await?;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
        Ok(())
    }
}
