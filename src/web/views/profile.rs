use std::collections::HashMap;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use either::Either;
use futures::TryStreamExt;
use serde::Deserialize;
use tracing::{info, instrument};

use crate::{
    models::{Anonymous, Rating, User, Vote},
    prelude::*,
    web::{error::WebError, partials::*, prelude::*, result::WebResult, state::AppState},
};

#[derive(Deserialize)]
pub struct GetParams {
    pub account_id: u32,
}

#[instrument(skip_all, fields(account_id = params.account_id))]
pub async fn get(
    Path(params): Path<GetParams>,
    user: Either<User, Anonymous>,
    State(state): State<AppState>,
) -> WebResult<impl IntoResponse> {
    let Either::Left(user) = user else {
        return Err(WebError::Unauthorized)
    };
    if params.account_id != user.account_id {
        return Err(WebError::Forbidden);
    }

    let vehicles_stats = state
        .get_vehicle_stats(user.account_id)
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
            (profile_navbar(&user))

            section.section {
                div.container {
                    div.columns.is-multiline.is-tablet {
                        @for stats in vehicles_stats.values() {
                            div.column."is-6-tablet"."is-4-desktop"."is-3-widescreen" {
                                (
                                    VehicleCard::new(stats.tank_id)
                                        .tankopedia(state.tankopedia.get(&stats.tank_id))
                                        .last_battle_time(stats.last_battle_time)
                                        .rating(user.account_id, votes.get(&stats.tank_id).copied())
                                )
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

#[derive(Deserialize)]
pub struct PostParams {
    pub account_id: u32,
    pub tank_id: u16,
}

#[inline]
pub async fn like_vehicle(
    state: State<AppState>,
    user: Either<User, Anonymous>,
    Path(params): Path<PostParams>,
) -> WebResult<impl IntoResponse> {
    rate_vehicle(state, user, params, Some(Rating::Like)).await
}

#[inline]
pub async fn dislike_vehicle(
    state: State<AppState>,
    user: Either<User, Anonymous>,
    Path(params): Path<PostParams>,
) -> WebResult<impl IntoResponse> {
    rate_vehicle(state, user, params, Some(Rating::Dislike)).await
}

#[inline]
pub async fn unrate_vehicle(
    state: State<AppState>,
    user: Either<User, Anonymous>,
    Path(params): Path<PostParams>,
) -> WebResult<impl IntoResponse> {
    rate_vehicle(state, user, params, None).await
}

#[instrument(skip_all, fields(account_id = params.account_id, tank_id = params.tank_id))]
async fn rate_vehicle(
    State(state): State<AppState>,
    user: Either<User, Anonymous>,
    params: PostParams,
    rating: Option<Rating>,
) -> WebResult<impl IntoResponse> {
    let Either::Left(user) = user else {
        return Err(WebError::Unauthorized)
    };
    if params.account_id != user.account_id {
        return Err(WebError::Forbidden);
    }
    if !state.owns_vehicle(user.account_id, params.tank_id).await? {
        return Err(WebError::ImATeapot);
    }

    info!(?rating);
    if let Some(rating) = rating {
        state
            .vote_manager
            .insert(&Vote::new(user.account_id, params.tank_id, rating))
            .await?;
    } else {
        state.vote_manager.delete(user.account_id, params.tank_id).await?;
    }

    state.purge_predictions(user.account_id).await;
    Ok(VehicleCard::vehicle_rate_buttons(user.account_id, params.tank_id, rating))
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    use super::*;
    use crate::{models::User, web::Web};

    #[tokio::test]
    #[ignore]
    async fn own_profile_ok() -> Result {
        let state = AppState::new_test().await?;
        let session_id = state.session_manager.insert_test_session().await?;
        let request = Request::builder()
            .uri("/profile/0")
            .header("Cookie", format!("{}={session_id}", User::SESSION_COOKIE_NAME))
            .body(Body::empty())?;
        let response = Web::create_router(state).oneshot(request).await?;
        assert_eq!(response.status(), StatusCode::OK);
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn reject_anonymous_profile_ok() -> Result {
        let app = Web::create_router(AppState::new_test().await?);
        let request = Request::builder().uri("/profile/0").body(Body::empty())?;
        let response = app.oneshot(request).await?;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn reject_anonymous_vote_ok() -> Result {
        let app = Web::create_router(AppState::new_test().await?);
        let request = Request::builder()
            .uri("/profile/0/vehicle/1/like")
            .method("POST")
            .body(Body::empty())?;
        let response = app.oneshot(request).await?;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn rate_own_vehicle_ok() -> Result {
        let state = AppState::new_test().await?;
        let session_id = state.session_manager.insert_test_session().await?;
        let request = Request::builder()
            .uri("/profile/0/vehicle/1/like")
            .method("POST")
            .header("Cookie", format!("{}={session_id}", User::SESSION_COOKIE_NAME))
            .body(Body::empty())?;
        let response = Web::create_router(state).oneshot(request).await?;
        assert_eq!(response.status(), StatusCode::OK);
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn reject_own_non_played_vehicle_ok() -> Result {
        let state = AppState::new_test().await?;
        let session_id = state.session_manager.insert_test_session().await?;
        let request = Request::builder()
            .uri("/profile/0/vehicle/2/like")
            .method("POST")
            .header("Cookie", format!("{}={session_id}", User::SESSION_COOKIE_NAME))
            .body(Body::empty())?;
        let response = Web::create_router(state).oneshot(request).await?;
        assert_eq!(response.status(), StatusCode::IM_A_TEAPOT);
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn reject_rate_others_vehicle_ok() -> Result {
        let state = AppState::new_test().await?;
        let session_id = state.session_manager.insert_test_session().await?;
        let request = Request::builder()
            .uri("/profile/1/vehicle/1/like")
            .method("POST")
            .header("Cookie", format!("{}={session_id}", User::SESSION_COOKIE_NAME))
            .body(Body::empty())?;
        let response = Web::create_router(state).oneshot(request).await?;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
        Ok(())
    }
}
