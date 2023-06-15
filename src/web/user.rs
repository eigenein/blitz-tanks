use std::str::FromStr;

use anyhow::Context;
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    headers,
    http::request::Parts,
    RequestPartsExt, TypedHeader,
};
use either::Either;
use tracing::{debug, instrument};
use uuid::Uuid;

use crate::{
    models::user::{Anonymous, User},
    tracing::configure_user,
    web::{prelude::*, state::AppState},
};

/// Extract a user from the request.
/// TODO: move to `extract`.
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
