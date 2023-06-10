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

/// Account ID path segment.
#[derive(Deserialize)]
pub struct AccountId(#[serde(rename = "account_id")] pub u32);

/// User extractor, which validates the account ID path segment.
/// In order to pass, the path's account ID **must** be the same as that of the logged in user.
pub struct Owner(pub User);

#[async_trait]
impl<S> FromRequestParts<S> for Owner
where
    S: Sync + Send,
    AppState: FromRef<S>,
{
    type Rejection = WebError;

    #[instrument(level = "debug", skip_all)]
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Path(AccountId(account_id)) =
            Path::<AccountId>::from_request_parts(parts, state).await?;
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
