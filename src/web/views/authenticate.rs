use axum::{
    extract::{Query, State},
    http::header::SET_COOKIE,
    response::{IntoResponse, Redirect},
};
use serde::Deserialize;
use tracing::{info, instrument};

use crate::{
    models::{new_session_id, User},
    prelude::*,
    web::{prelude::*, session::Session, state::AppState},
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

impl From<AuthenticationResult> for Result<User, WebError> {
    fn from(value: AuthenticationResult) -> Self {
        match value {
            AuthenticationResult::Ok(user) => Ok(user),
            AuthenticationResult::Err(error) => Err(WebError::ServiceUnavailable {
                code: error.code,
                message: error.message,
            }),
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
) -> WebResult<impl IntoResponse> {
    let user = Result::<User, WebError>::from(result)?;
    let session_id = new_session_id();
    info!(user.nickname, %session_id, "👋 welcome");
    state.session_manager.insert(session_id, &user)?;
    let cookie = cookie::Cookie::build(Session::SESSION_COOKIE_NAME, session_id.to_string())
        .http_only(true)
        .expires(user.expires_at()?)
        .finish();

    Ok((
        [(SET_COOKIE, cookie.to_string())],
        Redirect::temporary(&format!("/profile/{}", user.account_id)),
    ))
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
            .uri("/welcome?status=ok&access_token=fake&expires_at=1686693094&nickname=test&account_id=1")
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
            .uri("/welcome?status=error&code=500&message=ricochet")
            .body(Body::empty())?;
        let mut response = app.oneshot(request).await?;
        assert_eq!(
            response.status(),
            StatusCode::SERVICE_UNAVAILABLE,
            "{:?}",
            response.data().await
        );
        Ok(())
    }
}
