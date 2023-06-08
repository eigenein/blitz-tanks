//! Web-specific models.

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, Path},
    http::request::Parts,
};
use serde::Deserialize;

use crate::{db::Db, models::User, web::error::WebError};

#[derive(Deserialize)]
pub struct AccountId(#[serde(rename = "account_id")] pub u32);

/// Account ID extractor which validates the current session.
pub struct ValidatedAccountId(pub u32);

#[async_trait]
impl<S> FromRequestParts<S> for ValidatedAccountId
where
    S: Sync + Send,
    Db: FromRef<S>,
{
    type Rejection = WebError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Path(AccountId(account_id)) =
            Path::<AccountId>::from_request_parts(parts, state).await?;
        let user = User::from_request_parts(parts, state).await?;
        if user.account_id == account_id {
            Ok(Self(account_id))
        } else {
            Err(WebError::Forbidden)
        }
    }
}
