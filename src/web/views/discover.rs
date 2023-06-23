use axum::{extract::State, response::IntoResponse};
use maud::html;
use tracing::instrument;

use crate::web::{extract::ProfileOwner, result::WebResult, state::AppState, views::partials::*};

#[instrument(skip_all, fields(account_id = user.account_id))]
pub async fn get(
    ProfileOwner(user): ProfileOwner,
    State(state): State<AppState>,
) -> WebResult<impl IntoResponse> {
    let markup = html! {
        (head())
        body {
            (navbar(&user))

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
