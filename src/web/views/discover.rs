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

    let predictions = state.get_predictions(user.account_id).await?;

    let markup = html! {
        (head())
        body {
            (profile_navbar(&user))

            section.section {
                div.container {
                    h1.title { "Most liked by community" }

                    div.columns.is-multiline.is-tablet {
                        @for tank_id in state.model.biases.keys().take(6) {
                            div.column."is-6-tablet"."is-4-desktop"."is-3-widescreen"."is-2-fullhd" {
                                (
                                    VehicleCard::new(*tank_id)
                                        .tankopedia(state.tankopedia.get(tank_id))
                                        .title_style("is-6")
                                )
                            }
                        }
                    }
                }
            }

            section.section {
                div.container {
                    h1.title { "For you" }

                    div.columns.is-multiline.is-tablet {
                        @for (tank_id, rating) in predictions.iter() {
                            div.column."is-6-tablet"."is-4-desktop"."is-3-widescreen"."is-2-fullhd" {
                                (
                                    VehicleCard::new(*tank_id)
                                        .tankopedia(state.tankopedia.get(tank_id))
                                        .title_style("is-6")
                                        .predicted_rating(*rating)
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
