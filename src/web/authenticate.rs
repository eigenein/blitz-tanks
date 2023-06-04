//! Authentication redirect view.

use axum::{
    extract::{Query, State},
    http::header::SET_COOKIE,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    db::Db,
    models::User,
    prelude::*,
    web::{prelude::*, session::Session, state::AppState},
};

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

/// Handle Wargaming.net authentication redirect and start a new session.
pub async fn get(
    Query(result): Query<AuthenticationResult>,
    State(db): State<Db>,
) -> WebResult<impl IntoResponse> {
    let user = Result::from(result)?; // TODO: handle errors.
    let session_id = Uuid::now_v7();
    db.session_manager()?.insert(session_id, &user)?;

    // TODO: content.
    Ok((
        [(
            SET_COOKIE,
            format!(
                "{}={session_id}; HttpOnly; Expires={}",
                Session::SESSION_COOKIE_NAME,
                user.expires_at()?.to_rfc2822()
            ),
        )],
        "OK",
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
            .uri("/authenticate?status=ok&access_token=fake&expires_at=1686693094&nickname=test&account_id=1")
            .body(Body::empty())?;
        let mut response = app.oneshot(request).await?;
        assert_eq!(response.status(), StatusCode::OK, "{:?}", response.data().await);
        assert!(
            response
                .headers()
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
