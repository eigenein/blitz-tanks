use std::collections::HashMap;

use axum::{extract::State, response::IntoResponse};
use futures::TryStreamExt;
use tracing::{info, instrument};

use crate::{
    models::{rating::Rating, user::User, vote::Vote},
    prelude::*,
    web::{
        error::WebError,
        extract::{ProfileOwner, UserOwnedTank},
        prelude::*,
        result::WebResult,
        state::AppState,
        views::partials::*,
    },
};

#[instrument(skip_all, fields(account_id = user.account_id))]
pub async fn get(
    ProfileOwner(user): ProfileOwner,
    State(state): State<AppState>,
) -> WebResult<impl IntoResponse> {
    let vehicles_stats = state
        .vehicle_stats_getter
        .get(user.account_id)
        .await
        .map_err(WebError::ServiceUnavailable)?;
    let votes: HashMap<u16, Rating> = state
        .vote_manager
        .iter_by_account_id(user.account_id)
        .await?
        .map_ok(|vote| (vote.tank_id, vote.rating))
        .map_err(|error| WebError::InternalServerError(anyhow!(error)))
        .try_collect()
        .await?;

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
                                div.card {
                                    @let vehicle = state.tankopedia.get(&stats.tank_id);
                                    (vehicle_card_image(vehicle))
                                    (vehicle_card_content(vehicle, stats))
                                    (vehicle_card_footer(user.account_id, stats.tank_id, votes.get(&stats.tank_id).copied()))
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

#[inline]
pub async fn like_vehicle(
    state: State<AppState>,
    owned_tank: UserOwnedTank,
) -> WebResult<impl IntoResponse> {
    rate_vehicle(state, owned_tank, Some(Rating::Like)).await
}

#[inline]
pub async fn dislike_vehicle(
    state: State<AppState>,
    owned_tank: UserOwnedTank,
) -> WebResult<impl IntoResponse> {
    rate_vehicle(state, owned_tank, Some(Rating::Dislike)).await
}

#[inline]
pub async fn unrate_vehicle(
    state: State<AppState>,
    owned_tank: UserOwnedTank,
) -> WebResult<impl IntoResponse> {
    rate_vehicle(state, owned_tank, None).await
}

#[instrument(skip_all, fields(account_id = user.account_id, tank_id = tank_id))]
async fn rate_vehicle(
    State(state): State<AppState>,
    UserOwnedTank { user, tank_id }: UserOwnedTank,
    rating: Option<Rating>,
) -> WebResult<impl IntoResponse> {
    info!(?rating);

    let manager = state.vote_manager;
    if let Some(rating) = rating {
        manager.insert(&Vote::new(user.account_id, tank_id, rating)).await?;
    } else {
        manager.delete(user.account_id, tank_id).await?;
    }

    Ok(vehicle_card_footer(user.account_id, tank_id, rating))
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    use super::*;
    use crate::{
        prelude::Result,
        web::{state::AppState, Web},
    };

    #[tokio::test]
    async fn own_profile_ok() -> Result {
        let state = AppState::new_test().await?;
        let session_id = state.session_manager.insert_test_session().await?;
        let request = Request::builder()
            .uri("/profile/0")
            .header("Cookie", format!("{}={session_id}", User::SESSION_COOKIE_NAME))
            .body(Body::empty())?;
        let response = Web::create_app(state).oneshot(request).await?;
        assert_eq!(response.status(), StatusCode::OK);
        Ok(())
    }

    #[tokio::test]
    async fn reject_anonymous_profile_ok() -> Result {
        let app = Web::create_app(AppState::new_test().await?);
        let request = Request::builder().uri("/profile/0").body(Body::empty())?;
        let response = app.oneshot(request).await?;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
        Ok(())
    }

    #[tokio::test]
    async fn reject_anonymous_vote_ok() -> Result {
        let app = Web::create_app(AppState::new_test().await?);
        let request = Request::builder()
            .uri("/profile/0/vehicle/1/like")
            .method("POST")
            .body(Body::empty())?;
        let response = app.oneshot(request).await?;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
        Ok(())
    }

    #[tokio::test]
    async fn rate_own_vehicle_ok() -> Result {
        let state = AppState::new_test().await?;
        let session_id = state.session_manager.insert_test_session().await?;
        let request = Request::builder()
            .uri("/profile/0/vehicle/1/like")
            .method("POST")
            .header("Cookie", format!("{}={session_id}", User::SESSION_COOKIE_NAME))
            .body(Body::empty())?;
        let response = Web::create_app(state).oneshot(request).await?;
        assert_eq!(response.status(), StatusCode::OK);
        Ok(())
    }

    #[tokio::test]
    async fn reject_own_non_played_vehicle_ok() -> Result {
        let state = AppState::new_test().await?;
        let session_id = state.session_manager.insert_test_session().await?;
        let request = Request::builder()
            .uri("/profile/0/vehicle/2/like")
            .method("POST")
            .header("Cookie", format!("{}={session_id}", User::SESSION_COOKIE_NAME))
            .body(Body::empty())?;
        let response = Web::create_app(state).oneshot(request).await?;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
        Ok(())
    }

    #[tokio::test]
    async fn reject_rate_others_vehicle_ok() -> Result {
        let state = AppState::new_test().await?;
        let session_id = state.session_manager.insert_test_session().await?;
        let request = Request::builder()
            .uri("/profile/1/vehicle/1/like")
            .method("POST")
            .header("Cookie", format!("{}={session_id}", User::SESSION_COOKIE_NAME))
            .body(Body::empty())?;
        let response = Web::create_app(state).oneshot(request).await?;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
        Ok(())
    }
}
