use axum::{
    extract::{Query, State},
    http::header::SET_COOKIE,
    response::IntoResponse,
};
use cookie::SameSite;
use maud::{html, DOCTYPE};
use serde::Deserialize;
use tracing::{info, instrument};
use uuid::Uuid;

use crate::{
    models::User,
    prelude::*,
    web::{
        error::{ForbiddenReason, WebError},
        result::WebResult,
        state::AppState,
    },
};

/// Wargaming.net [authentication redirect][1] query parameters.
///
/// [1]: https://developers.wargaming.net/reference/all/wot/auth/login/
#[serde_with::serde_as]
#[derive(Deserialize)]
#[serde(tag = "status")]
pub enum AuthenticationResult {
    #[serde(rename = "ok")]
    Ok {
        access_token: String,

        #[serde_as(as = "serde_with::DisplayFromStr")]
        expires_at: i64,

        #[serde_as(as = "serde_with::DisplayFromStr")]
        account_id: u32,

        nickname: String,
    },

    #[serde(rename = "error")]
    Err {
        #[serde_as(as = "serde_with::DisplayFromStr")]
        code: u16,

        message: String,
    },
}

impl From<AuthenticationResult> for WebResult<User> {
    fn from(value: AuthenticationResult) -> Self {
        match value {
            AuthenticationResult::Ok {
                access_token,
                expires_at,
                account_id,
                nickname,
            } => Ok(User {
                session_id: Uuid::new_v4(),
                access_token,
                expires_at: Utc
                    .timestamp_opt(expires_at, 0)
                    .single()
                    .ok_or_else(|| anyhow!("incorrect expiration timestamp `{expires_at}`"))?,
                account_id,
                nickname,
            }),
            AuthenticationResult::Err { code, message } => Err(WebError::ServiceUnavailable(
                anyhow!("Wargaming.net API error {code}/{message}"),
            )),
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

    // Verify the sign-in.
    let Some(account_info) = state.wg.get_account_info(user.account_id, &user.access_token).await? else {
        return Err(WebError::Forbidden(ForbiddenReason::NonExistentAccount));
    };
    if account_info.private.is_null() {
        return Err(WebError::Forbidden(ForbiddenReason::NoPrivateAccess));
    }

    info!(user.nickname, %user.session_id, "👋 Welcome");
    state.session_manager.insert(&user).await?;
    let cookie = cookie::Cookie::build(User::SESSION_COOKIE_NAME, user.session_id.to_string())
        .http_only(true)
        .expires(user.expires_at()?)
        .same_site(SameSite::Strict)
        .secure(true)
        .finish();

    // Workaround for Chrome & Firefox not sending the cookie after the redirect.
    let markup = html! {
        (DOCTYPE)
        html {
            head { meta http-equiv="refresh" content=(format!("0; url='/profile/{}'", user.account_id)); }
            body {}
        }
    };
    Ok(([(SET_COOKIE, cookie.to_string())], markup))
}

#[cfg(test)]
mod tests {
    use axum::{
        body::{Body, HttpBody},
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    use super::*;
    use crate::web::{state::AppState, Web};

    #[tokio::test]
    #[ignore]
    async fn success_ok() -> Result {
        let app = Web::create_router(AppState::new_test().await?);
        let request = Request::builder()
            .uri("/welcome?status=ok&access_token=fake&expires_at=1686693094&nickname=test&account_id=1")
            .body(Body::empty())?;
        let mut response = app.oneshot(request).await?;
        assert_eq!(response.status(), StatusCode::OK);
        let headers = response.headers();
        assert!(headers.get(SET_COOKIE).unwrap().to_str()?.contains(User::SESSION_COOKIE_NAME));
        let body = String::from_utf8(response.body_mut().data().await.unwrap()?.to_vec())?;
        assert!(body.contains(r#"content="0; url='/profile/1'""#));
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn error_ok() -> Result {
        let app = Web::create_router(AppState::new_test().await?);
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
