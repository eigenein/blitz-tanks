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
use scru128::Scru128Id;
use tracing::{debug, instrument};

use crate::{
    models::{Anonymous, LegacyUser, User},
    tracing::configure_user,
    web::{prelude::*, state::AppState},
};

/// Extract a user from the request.
#[async_trait]
impl FromRequestParts<AppState> for Either<LegacyUser, Anonymous> {
    type Rejection = WebError;

    #[instrument(level = "debug", skip_all)]
    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let cookie: Option<TypedHeader<headers::Cookie>> = parts.extract().await?;
        let Some(cookie) = cookie else { return Ok(Either::Right(Anonymous)) };
        let Some(session_id) = cookie.get(User::SESSION_COOKIE_NAME) else { return Ok(Either::Right(Anonymous)) };
        debug!(session_id, "🔑 authenticating…");
        let session_id = Scru128Id::from_str(session_id)
            .context("malformed session ID")
            .map_err(WebError::BadRequest)?;

        sentry::configure_scope(|scope| scope.set_tag("user.session_id", session_id));

        match AppState::from_ref(state).session_manager.get(session_id)? {
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

/// Extract a user from the request or reject if there's no any session.
#[async_trait]
impl FromRequestParts<AppState> for LegacyUser {
    type Rejection = WebError;

    #[instrument(level = "debug", skip_all)]
    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        match Either::<LegacyUser, Anonymous>::from_request_parts(parts, state).await? {
            Either::Left(user) => Ok(user),
            Either::Right(_) => Err(WebError::Forbidden),
        }
    }
}
