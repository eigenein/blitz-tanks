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
                        @for vehicle_id in state.model.top_vehicles.iter() {
                            div.column."is-4-tablet"."is-3-desktop"."is-2-widescreen" {
                                div.card {
                                    @let vehicle = state.tankopedia.get(vehicle_id);
                                    (vehicle_card_image(vehicle))
                                    (vehicle_card_content(*vehicle_id, vehicle, None, "is-6"))
                                }
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
