use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, Path},
    http::request::Parts,
};
use serde::Deserialize;
use tracing::{debug, instrument, warn};

use crate::{
    models::User,
    web::{prelude::*, state::AppState},
};

/// User extractor, which validates the account ID path segment.
/// In order to pass, the path's account ID **must** be the same as that of the logged in user.
pub struct ProfileOwner(pub User);

#[async_trait]
impl<S> FromRequestParts<S> for ProfileOwner
where
    S: Sync + Send,
    AppState: FromRef<S>,
{
    type Rejection = WebError;

    #[instrument(level = "debug", skip_all)]
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        #[derive(Deserialize)]
        pub struct PathParams {
            pub account_id: u32,
        }

        let Path(PathParams { account_id }) = Path::from_request_parts(parts, state).await?;
        let user = User::from_request_parts(parts, state).await?;

        if user.account_id == account_id {
            debug!(account_id, "✅ verified");
            Ok(Self(user))
        } else {
            warn!(account_id, "❌ forbidden");
            Err(WebError::Forbidden)
        }
    }
}

pub struct ProfileOwnedTank {
    pub tank_id: u16,
    pub user: User,
}

#[async_trait]
impl FromRequestParts<AppState> for ProfileOwnedTank {
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
        let user = User::from_request_parts(parts, state).await?;

        if params.account_id == user.account_id
            && state
                .vehicle_stats_getter
                .owns_vehicle(params.account_id, params.tank_id)
                .await?
        {
            debug!(params.account_id, params.tank_id, "✅ verified");
            Ok(Self { tank_id: params.tank_id, user })
        } else {
            warn!(params.account_id, params.tank_id, "❌ forbidden");
            Err(WebError::Forbidden)
        }
    }
}
