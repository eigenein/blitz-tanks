use axum::{
    async_trait,
    extract::{FromRequestParts, Path},
    http::request::Parts,
};
use either::Either;
use serde::Deserialize;
use tracing::{debug, instrument, warn};

use crate::{
    models::user::{Anonymous, User},
    web::{prelude::*, state::AppState},
};

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
