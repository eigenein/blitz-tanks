use axum::{extract::State, response::IntoResponse};
use either::Either;
use maud::html;
use tracing::instrument;

use crate::{
    models::user::{Anonymous, User},
    web::{error::WebError, result::WebResult, state::AppState, views::partials::*},
};

#[instrument(skip_all)]
pub async fn get(
    user: Either<User, Anonymous>,
    State(state): State<AppState>,
) -> WebResult<impl IntoResponse> {
    let Either::Left(user) = user else { return Err(WebError::Unauthorized) };

    let markup = html! {
        (head())
        body {
            (profile_navbar(&user))

            section.section {
                div.container {
                    div.columns.is-multiline.is-tablet {}
                }
            }

            (footer())
        }
    };
    Ok(markup)
}
