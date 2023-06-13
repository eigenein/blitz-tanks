use std::str::FromStr;

use anyhow::Context;
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    headers,
    http::request::Parts,
    RequestPartsExt, TypedHeader,
};
use scru128::Scru128Id;
use tracing::{debug, instrument};

use crate::{
    models::User,
    tracing::configure_user,
    web::{prelude::*, state::AppState},
};

/// Client-side session.
pub enum Session {
    Authenticated(User),

    /// Unidentified user: the session cookie is either missing, expired, or invalid.
    Anonymous,
}

/// Extract a session from the request.
#[async_trait]
impl<S> FromRequestParts<S> for Session
where
    AppState: FromRef<S>,
    S: Sync,
{
    type Rejection = WebError;

    #[instrument(level = "debug", skip_all)]
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let cookie: Option<TypedHeader<headers::Cookie>> = parts.extract().await?;
        let Some(cookie) = cookie else { return Ok(Session::Anonymous) };
        let Some(session_id) = cookie.get(User::SESSION_COOKIE_NAME) else { return Ok(Session::Anonymous) };
        debug!(session_id, "🔑 authenticating…");
        let session_id = Scru128Id::from_str(session_id)
            .context("malformed session ID")
            .map_err(WebError::BadRequest)?;

        sentry::configure_scope(|scope| scope.set_tag("user.session_id", session_id));

        match AppState::from_ref(state).session_manager.get(session_id)? {
            Some(user) => {
                sentry::configure_scope(|scope| configure_user(scope, Some(&user)));
                Ok(Session::Authenticated(user))
            }
            None => {
                sentry::configure_scope(|scope| configure_user(scope, None));
                Ok(Session::Anonymous)
            }
        }
    }
}

/// Extract a user from the request or reject if there's no any session.
#[async_trait]
impl<S> FromRequestParts<S> for User
where
    AppState: FromRef<S>,
    S: Sync,
{
    type Rejection = WebError;

    #[instrument(level = "debug", skip_all)]
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match Session::from_request_parts(parts, state).await? {
            Session::Authenticated(user) => Ok(user),
            Session::Anonymous => Err(WebError::Forbidden),
        }
    }
}
