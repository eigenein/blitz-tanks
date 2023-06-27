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
                    h1.title { "Most liked" }

                    div.columns.is-multiline.is-tablet {
                        @for vehicle_id in state.model.biases.keys().take(6) {
                            div.column."is-6-tablet"."is-4-desktop"."is-3-widescreen"."is-2-fullhd" {
                                (
                                    VehicleCard::new(*vehicle_id)
                                        .tankopedia(state.tankopedia.get(vehicle_id))
                                        .title_style("is-6")
                                )
                            }
                        }
                    }
                }
            }

            (footer())
        }
    };
    Ok(markup)
}
