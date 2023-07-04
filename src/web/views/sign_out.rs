use axum::{
    extract::State,
    http::header::SET_COOKIE,
    response::{IntoResponse, Redirect},
};
use cookie::time::Duration;
use either::Either;
use sentry::capture_error;
use tracing::{error, info, instrument};

use crate::{
    models::{Anonymous, User},
    web::{result::WebResult, state::AppState},
};

#[instrument(skip_all)]
pub async fn get(
    user: Either<User, Anonymous>,
    State(state): State<AppState>,
) -> WebResult<impl IntoResponse> {
    if let Either::Left(user) = user {
        info!(user.account_id, user.nickname, "😿 Bye!");
        match state.wg.log_out(&user.access_token).await {
            Ok(_) => {
                info!(user.account_id, "✅ The access token has been successfully revoked");
            }
            Err(error) => {
                error!(user.access_token, "⚠️ Failed to log out: {:#}", error);
                capture_error(&error);
            }
        }
    }

    let cookie = cookie::Cookie::build(User::SESSION_COOKIE_NAME, "")
        .http_only(true)
        .max_age(Duration::new(0, 0))
        .finish();

    Ok(([(SET_COOKIE, cookie.to_string())], Redirect::temporary("/")))
}
