use std::collections::HashMap;

use axum::extract::State;
use chrono::LocalResult;
use chrono_humanize::HumanTime;
use tracing::{info, instrument};

use crate::{
    models::{LegacyUser, Rating, Vote},
    prelude::*,
    web::{
        extract::{ProfileOwner, UserOwnedTank},
        prelude::*,
        state::AppState,
        views::partials::*,
    },
    wg::stats::VehicleStats,
};

#[instrument(skip_all, fields(account_id = user.account_id))]
pub async fn get(
    ProfileOwner(user): ProfileOwner,
    State(state): State<AppState>,
) -> WebResult<impl IntoResponse> {
    let vehicles_stats = state.vehicle_stats_getter.get(user.account_id).await?;
    let votes: HashMap<u16, Rating> = state
        .vote_manager
        .get_all_by_account_id(user.account_id)?
        .into_iter()
        .map(|(tank_id, vote)| (tank_id, vote.rating()))
        .collect();

    let markup = html! {
        (head())
        body {
            (navbar(&user))

            section.section {
                div.container {
                    article.message.is-warning {
                        div.message-body.content {
                            div.content {
                                p {
                                    "Unfortunately, "
                                    strong { "some pictures and vehicles names are missing" }
                                    " due to the Wargaming.net API being incomplete."
                                }
                                p {
                                    "Eventually, this will be somehow fixed, but for now "
                                    strong { "there are links to Armor Inspector" }
                                    " next to the vehicle titles, which can be used to look up a certain vehicle."
                                }
                            }
                        }
                    }

                    div.columns.is-multiline.is-tablet {
                        @for stats in vehicles_stats.values() {
                            div.column."is-6-tablet"."is-4-desktop"."is-3-widescreen" {
                                @let account_id = user.account_id;
                                (vehicle_card(&state, account_id, stats, votes.get(&stats.tank_id).copied())?)
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

    let manager = state.vote_manager;
    if let Some(rating) = rating {
        manager.insert(user.account_id, tank_id, &Vote::new_now(rating))?;
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
            @let is_premium = description.is_some_and(|d| d.is_premium);

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
                            span.icon-text {
                                span {
                                    @match description {
                                        Some(description) => { span.has-text-warning-dark[is_premium] { (description.name) } },
                                        None => { "#" (stats.tank_id) },
                                    }
                                }
                                span.icon {
                                    a
                                        title="View in Armor Inspector"
                                        href=(format!("https://armor.wotinspector.com/en/blitz/{}-/", stats.tank_id))
                                    {
                                        i.fa-solid.fa-arrow-up-right-from-square {}
                                    }
                                }
                            }
                        }
                        @if let LocalResult::Single(timestamp) = stats.last_battle_time() {
                            p.subtitle."is-6" {
                                span.has-text-grey { "Last played" }
                                " "
                                span.has-text-weight-medium title=(timestamp) { (HumanTime::from(timestamp)) }
                            }
                        }
                    }
                }
            }

            footer.card-footer {
                (vehicle_card_footer(account_id, stats.tank_id, rating))
            }
        }
    };
    Ok(markup)
}

/// Render the vehicle card's footer inner HTML.
///
/// # Notes
///
/// It's extracted for HTMX to be able to refresh the voting buttons.
fn vehicle_card_footer(account_id: u32, tank_id: u16, rating: Option<Rating>) -> Markup {
    html! {
        a.card-footer-item.has-background-success-light[rating == Some(Rating::Like)]
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
        a.card-footer-item.has-background-danger-light[rating == Some(Rating::Dislike)]
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
fn navbar(LegacyUser { account_id, nickname, .. }: &LegacyUser) -> Markup {
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

                        // a.navbar-item href=(format!("/profile/{account_id}/discover")) {
                        //     span.icon { i.fa-solid.fa-wand-magic-sparkles aria-hidden="true" {} }
                        //     span { "Discover" }
                        // }
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
        models::User,
        prelude::Result,
        web::{create_app, state::AppState},
    };

    #[tokio::test]
    #[ignore]
    async fn own_profile_ok() -> Result {
        let state = AppState::new_test().await?;
        let session_id = state.session_manager.insert_test_session()?;
        let request = Request::builder()
            .uri("/profile/0")
            .header("Cookie", format!("{}={session_id}", User::SESSION_COOKIE_NAME))
            .body(Body::empty())?;
        let response = create_app(state).oneshot(request).await?;
        assert_eq!(response.status(), StatusCode::OK);
        Ok(())
    }

    #[tokio::test]
    async fn reject_anonymous_profile_ok() -> Result {
        let app = create_app(AppState::new_test().await?);
        let request = Request::builder().uri("/profile/0").body(Body::empty())?;
        let response = app.oneshot(request).await?;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
        Ok(())
    }
}
