use axum::response::IntoResponse;
use tracing::instrument;

use crate::{
    prelude::*,
    web::{error::WebError, models::ValidatedAccountId},
};

#[instrument(skip_all, fields(account_id = account_id))]
pub async fn get(
    ValidatedAccountId(account_id): ValidatedAccountId,
) -> Result<impl IntoResponse, WebError> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    use super::*;
    use crate::{
        models::User,
        prelude::Result,
        web::{authenticate::Session, create_app, state::AppState},
    };

    #[tokio::test]
    async fn own_profile_ok() -> Result {
        let state = AppState::new_test()?;

        let session_id = Session::new_id();
        state.db.session_manager()?.insert(
            session_id,
            &User {
                access_token: "test".to_string(),
                expires_at: Utc::now().timestamp() + 10,
                account_id: 1,
                nickname: "test".to_string(),
            },
        )?;

        let request = Request::builder()
            .uri("/profile/1")
            .header("Cookie", format!("{}={session_id}", Session::SESSION_COOKIE_NAME))
            .body(Body::empty())?;
        let response = create_app(state).oneshot(request).await?;
        assert_eq!(response.status(), StatusCode::OK);
        Ok(())
    }

    #[tokio::test]
    async fn reject_anonymous_profile_ok() -> Result {
        let app = create_app(AppState::new_test()?);
        let request = Request::builder().uri("/profile/1").body(Body::empty())?;
        let response = app.oneshot(request).await?;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
        Ok(())
    }
}
