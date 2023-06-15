use std::str::FromStr;

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, Path},
    headers,
    http::request::Parts,
    RequestPartsExt, TypedHeader,
};
use either::Either;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    models::user::{Anonymous, User},
    prelude::*,
    tracing::configure_user,
    web::{prelude::*, state::AppState},
};

/// Extract a user from the request.
#[async_trait]
impl FromRequestParts<AppState> for Either<User, Anonymous> {
    type Rejection = WebError;

    #[instrument(level = "debug", skip_all)]
    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let cookie: Option<TypedHeader<headers::Cookie>> = parts.extract().await?;
        let Some(cookie) = cookie else { return Ok(Either::Right(Anonymous)) };
        let Some(session_id) = cookie.get(User::SESSION_COOKIE_NAME) else { return Ok(Either::Right(Anonymous)) };
        debug!(session_id, "🔑 Authenticating…");
        let session_id = Uuid::from_str(session_id)
            .context("malformed session ID")
            .map_err(WebError::BadRequest)?;

        sentry::configure_scope(|scope| scope.set_tag("user.session_id", session_id));

        match AppState::from_ref(state).session_manager.get(session_id).await? {
            Some(user) => {
                sentry::configure_scope(|scope| configure_user(scope, Some(&user)));
                Ok(Either::Left(user))
            }
            None => {
                sentry::configure_scope(|scope| configure_user(scope, None));
                Ok(Either::Right(Anonymous))
            }
        }
    }
}

/// User extractor, which validates the account ID path segment.
/// In order to pass, the path's account ID **must** be the same as that of the logged in user.
pub struct ProfileOwner(pub User);

#[async_trait]
impl FromRequestParts<AppState> for ProfileOwner {
    type Rejection = WebError;

    #[instrument(level = "debug", skip_all)]
    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        #[derive(Deserialize)]
        pub struct PathParams {
            pub account_id: u32,
        }

        let Path(PathParams { account_id }) = Path::from_request_parts(parts, state).await?;
        let user = Either::<User, Anonymous>::from_request_parts(parts, state).await?;

        match user {
            Either::Left(user) if user.account_id == account_id => {
                debug!(account_id, "✅ Verified");
                Ok(Self(user))
            }
            _ => {
                warn!(account_id, "❌ Forbidden");
                Err(WebError::Forbidden)
            }
        }
    }
}

/// Vehicle owned by a certain user.
///
/// Validates that the user is the one logged in, and does own the vehicle.
pub struct UserOwnedTank {
    pub tank_id: u16,
    pub user: User,
}

#[async_trait]
impl FromRequestParts<AppState> for UserOwnedTank {
    type Rejection = WebError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        #[derive(Deserialize)]
        pub struct PathParams {
            pub account_id: u32,
            pub tank_id: u16,
        }

        let Path(params) = Path::<PathParams>::from_request_parts(parts, state).await?;
        let user = Either::<User, Anonymous>::from_request_parts(parts, state).await?;

        match user {
            Either::Left(user)
                if user.account_id == params.account_id
                    && state
                        .vehicle_stats_getter
                        .owns_vehicle(params.account_id, params.tank_id)
                        .await? =>
            {
                debug!(params.account_id, params.tank_id, "✅ Verified");
                Ok(Self { tank_id: params.tank_id, user })
            }
            _ => {
                warn!(params.account_id, params.tank_id, "❌ Forbidden");
                Err(WebError::Forbidden)
            }
        }
    }
}
