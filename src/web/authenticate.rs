use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, Query, State},
    headers,
    http::{header::SET_COOKIE, request::Parts},
    response::{IntoResponse, Redirect},
    RequestPartsExt, TypedHeader,
};
use serde::Deserialize;
use tracing::{info, instrument};
use uuid::Uuid;

use crate::{
    models::User,
    prelude::*,
    tracing::configure_user,
    web::{error::WebError, state::AppState},
};

/// Wargaming.net redirect query parameters.
#[derive(Deserialize)]
#[serde(tag = "status")]
pub enum AuthenticationResult {
    #[serde(rename = "ok")]
    Ok(User),

    #[serde(rename = "error")]
    Err(AuthenticationError),
}

#[serde_with::serde_as]
#[derive(Deserialize)]
pub struct AuthenticationError {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    code: u16,

    message: String,
}

impl From<AuthenticationResult> for Result<User> {
    fn from(value: AuthenticationResult) -> Self {
        match value {
            AuthenticationResult::Ok(user) => Ok(user),
            AuthenticationResult::Err(error) => {
                Err(anyhow!("authentication error #{}: {}", error.code, error.message))
            }
        }
    }
}

/// Handle [Wargaming.net authentication redirect][1] and start a new session.
///
/// [1]: https://developers.wargaming.net/reference/all/wot/auth/login/
#[instrument(skip_all)]
pub async fn get(
    Query(result): Query<AuthenticationResult>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, WebError> {
    let user = Result::from(result)?;
    let session_id = Session::new_id();
    info!(user.nickname, %session_id, "welcome");
    state.db.session_manager()?.insert(session_id, &user)?;
    let cookie = cookie::Cookie::build(Session::SESSION_COOKIE_NAME, session_id.to_string())
        .http_only(true)
        .expires(user.expires_at()?)
        .finish();

    Ok((
        [(SET_COOKIE, cookie.to_string())],
        Redirect::temporary(&format!("/profile/{}", user.account_id)),
    ))
}

/// Client-side session.
pub enum Session {
    Authenticated(User),

    /// Unidentified user: the session cookie is either missing, expired, or invalid.
    Anonymous,
}

impl Session {
    pub const SESSION_COOKIE_NAME: &'static str = "blitzTanksSessionId";

    #[instrument(level = "debug", ret)]
    pub fn new_id() -> Uuid {
        // UUID v7 is timestamp-based, so makes it easier to purge old sessions from the database.
        Uuid::now_v7()
    }
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
        let Some(session_id) = cookie.get(Self::SESSION_COOKIE_NAME) else { return Ok(Session::Anonymous) };
        let session_id = Uuid::parse_str(session_id).context("invalid session ID")?;

        sentry::configure_scope(|scope| scope.set_tag("user.session_id", session_id));

        match AppState::from_ref(state)
            .db
            .session_manager()?
            .get(session_id)?
        {
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

#[cfg(test)]
mod tests {
    use axum::{
        body::{Body, HttpBody},
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    use super::*;
    use crate::web::{create_app, state::AppState};

    #[tokio::test]
    async fn success_ok() -> Result {
        let app = create_app(AppState::new_test()?);
        let request = Request::builder()
            .uri("/authenticate?status=ok&access_token=fake&expires_at=1686693094&nickname=test&account_id=1")
            .body(Body::empty())?;
        let response = app.oneshot(request).await?;
        assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);
        let headers = response.headers();
        assert_eq!(headers.get("Location").unwrap(), "/profile/1");
        assert!(
            headers
                .get("Set-Cookie")
                .unwrap()
                .to_str()?
                .contains(Session::SESSION_COOKIE_NAME)
        );
        Ok(())
    }

    #[tokio::test]
    async fn error_ok() -> Result {
        let app = create_app(AppState::new_test()?);
        let request = Request::builder()
            .uri("/authenticate?status=error&code=500&message=ricochet")
            .body(Body::empty())?;
        let mut response = app.oneshot(request).await?;
        assert_eq!(
            response.status(),
            StatusCode::INTERNAL_SERVER_ERROR,
            "{:?}",
            response.data().await
        );
        Ok(())
    }
}
