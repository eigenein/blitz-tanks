use std::collections::HashMap;

use axum::extract::State;
use chrono::LocalResult;
use chrono_humanize::HumanTime;
use tracing::{info, instrument};

use crate::{
    models::{Rating, RatingEvent, User},
    prelude::*,
    web::{
        extract::{ProfileOwner, UserOwnedTank},
        prelude::*,
        state::AppState,
        views::partials::*,
    },
    weegee::VehicleStats,
};

#[instrument(skip_all, fields(account_id = user.account_id))]
pub async fn get(
    ProfileOwner(user): ProfileOwner,
    State(state): State<AppState>,
) -> WebResult<impl IntoResponse> {
    let vehicles_stats = state.vehicle_stats_getter.get(user.account_id).await?;
    let ratings: HashMap<u16, Rating> = state
        .rating_manager
        .get_all(user.account_id)?
        .into_iter()
        .map(|(tank_id, event)| (tank_id, event.rating()))
        .collect();

    let markup = html! {
        (head())
        body {
            (navbar(&user))

            section.section {
                div.container {
                    div.columns.is-multiline.is-tablet {
                        @for stats in vehicles_stats.values() {
                            div.column."is-6-tablet"."is-4-desktop"."is-3-widescreen" {
                                @let account_id = user.account_id;
                                (vehicle_card(&state, account_id, stats, ratings.get(&stats.tank_id).copied())?)
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

#[inline]
pub async fn like_vehicle(
    state: State<AppState>,
    owned_tank: UserOwnedTank,
) -> WebResult<impl IntoResponse> {
    post(state, owned_tank, Some(Rating::Like)).await
}

#[inline]
pub async fn dislike_vehicle(
    state: State<AppState>,
    owned_tank: UserOwnedTank,
) -> WebResult<impl IntoResponse> {
    post(state, owned_tank, Some(Rating::Dislike)).await
}

#[inline]
pub async fn unrate_vehicle(
    state: State<AppState>,
    owned_tank: UserOwnedTank,
) -> WebResult<impl IntoResponse> {
    post(state, owned_tank, None).await
}

/// Rate the vehicle.
#[instrument(skip_all, fields(account_id = user.account_id, tank_id = tank_id))]
async fn post(
    State(state): State<AppState>,
    UserOwnedTank { user, tank_id }: UserOwnedTank,
    rating: Option<Rating>,
) -> WebResult<impl IntoResponse> {
    info!(?rating);

    let manager = state.rating_manager;
    if let Some(rating) = rating {
        manager.insert(user.account_id, tank_id, &RatingEvent::new_now(rating))?;
    } else {
        manager.delete(user.account_id, tank_id)?;
    }

    Ok(vehicle_card_footer(user.account_id, tank_id, rating))
}

/// Render the vehicle card.
fn vehicle_card(
    state: &AppState,
    account_id: u32,
    stats: &VehicleStats,
    rating: Option<Rating>,
) -> Result<Markup> {
    let description = state.tankopedia.get(&stats.tank_id);
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
                                None => { "#" (stats.tank_id) },
                            }
                        }
                        @if let LocalResult::Single(timestamp) = stats.last_battle_time() {
                            p.subtitle."is-6" {
                                span.has-text-grey { "Last played" }
                                " "
                                span title=(timestamp) { (HumanTime::from(timestamp)) }
                            }
                        }
                    }
                }
            }

            footer.card-footer {
                (vehicle_card_footer(account_id, stats.tank_id, rating)) // TODO: actual rating.
            }
        }
    };
    Ok(markup)
}

/// Render the vehicle card's footer inner HTML.
///
/// # Notes
///
/// It's extracted for HTMX to be able to refresh the rating buttons.
fn vehicle_card_footer(account_id: u32, tank_id: u16, rating: Option<Rating>) -> Markup {
    html! {
        a.card-footer-item
            data-hx-post=(
                if rating != Some(Rating::Like) {
                    format!("/profile/{account_id}/vehicle/{tank_id}/like")
                } else {
                    format!("/profile/{account_id}/vehicle/{tank_id}/unrate")
                }
            )
            data-hx-target="closest .card-footer"
        {
            span.icon-text.has-text-success[rating == Some(Rating::Like)] {
                span.icon { i.fa-solid.fa-thumbs-up {} }
                span { "Like" }
            }
        }
        a.card-footer-item
            data-hx-post=(
                if rating != Some(Rating::Dislike) {
                    format!("/profile/{account_id}/vehicle/{tank_id}/dislike")
                } else {
                    format!("/profile/{account_id}/vehicle/{tank_id}/unrate")
                }
            )
            data-hx-target="closest .card-footer"
        {
            span.icon-text.has-text-danger[rating == Some(Rating::Dislike)] {
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
