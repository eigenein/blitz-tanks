use axum::extract::State;
use tracing::instrument;

use crate::{
    models::{RateAction, User},
    prelude::*,
    web::{
        extract::{ProfileOwnedTank, ProfileOwner},
        prelude::*,
        state::AppState,
        views::partials::*,
    },
};

#[instrument(skip_all, fields(account_id = user.account_id))]
pub async fn get(
    ProfileOwner(user): ProfileOwner,
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
                            div.column."is-6-tablet"."is-4-desktop"."is-3-widescreen" {
                                (vehicle_card(&state, user.account_id, stats.tank_id)?)
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

pub async fn like_vehicle(owned_tank: ProfileOwnedTank) -> WebResult<impl IntoResponse> {
    post(owned_tank, RateAction::Like).await
}

pub async fn dislike_vehicle(owned_tank: ProfileOwnedTank) -> WebResult<impl IntoResponse> {
    post(owned_tank, RateAction::Dislike).await
}

/// Rate the vehicle.
#[instrument(skip_all, fields(account_id = user.account_id, tank_id = tank_id))]
async fn post(
    ProfileOwnedTank { user, tank_id }: ProfileOwnedTank,
    action: RateAction,
) -> WebResult<impl IntoResponse> {
    Ok(vehicle_card_footer(user.account_id, tank_id)) // TODO
}

/// Render the vehicle card.
fn vehicle_card(state: &AppState, account_id: u32, tank_id: u16) -> Result<Markup> {
    let description = state.tankopedia.get(&tank_id);
    let markup = html! {
        div.card {
            div.card-image {
                figure.image {
                    @let url = description
                        .and_then(|d| d.images.normal_url.as_ref())
                        .map_or("https://dummyimage.com/1060x774", |url| url.as_str());
                    img src=(url) loading="lazy";
                }
            }

            div.card-content {
                div.media {
                    div.media-content {
                        p.title."is-5" {
                            @match description {
                                Some(description) => { (description.name) },
                                None => { "#" (tank_id) },
                            }
                        }
                    }
                }
            }

            footer.card-footer {
                (vehicle_card_footer(account_id, tank_id))
            }
        }
    };
    Ok(markup)
}

/// Render the vehicle card's footer inner HTML.
fn vehicle_card_footer(account_id: u32, tank_id: u16) -> Markup {
    html! {
        a.card-footer-item
            title="Hate it!"
            data-hx-post=(format!("/profile/{account_id}/vehicle/{tank_id}/like"))
            data-hx-target="closest .card-footer"
        {
            span.icon-text {
                span.icon { i.fa-solid.fa-thumbs-up {} }
                span { "Like" }
            }
        }
        a.card-footer-item
            title="Dislike it"
            data-hx-post=(format!("/profile/{account_id}/vehicle/{tank_id}/dislike"))
            data-hx-target="closest .card-footer"
        {
            span.icon-text {
                span.icon { i.fa-solid.fa-thumbs-down {} }
                span { "Dislike" }
            }
        }
    }
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
                                    a.button.is-rounded.is-danger href="/sign-out" {
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
        web::{create_app, session::Session, state::AppState},
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
