use axum::{extract::State, response::IntoResponse};
use either::Either;
use maud::html;
use tracing::instrument;

use crate::{
    cli::is_flag_set,
    models::{Anonymous, RatedTankId, User},
    tankopedia::vendored::TANKOPEDIA,
    web::{error::WebError, partials::*, result::WebResult, state::AppState},
};

#[instrument(skip_all)]
pub async fn get(
    user: Either<User, Anonymous>,
    State(state): State<AppState>,
) -> WebResult<impl IntoResponse> {
    // TODO: anonymous account should see the «most liked» section and a banner.
    let Either::Left(user) = user else {
        return Err(WebError::Unauthorized)
    };

    let biases = &state.model.load().biases;
    let predictions = state.get_predictions(user.account_id).await?;

    let markup = html! {
        (head())
        body {
            (profile_navbar(&user))

            @if !is_flag_set("BLITZ_TANKS_DISABLE_DISCOVER_MOST_LIKED")? {
                section.section {
                    div.container {
                        h1.title { "Most liked by community" }

                        div.columns.is-multiline.is-tablet {
                            @for tank_id in biases.keys().take(6) {
                                div.column."is-6-tablet"."is-4-desktop"."is-3-widescreen"."is-2-fullhd" {
                                    (VehicleCard::new(&TANKOPEDIA[tank_id]).title_style("is-6"))
                                }
                            }
                        }
                    }
                }
            }

            section.section {
                div.container {
                    h1.title { "For you" }

                    div.columns.is-multiline.is-tablet {
                        @for RatedTankId(tank_id, rating) in predictions.iter() {
                            div.column."is-6-tablet"."is-4-desktop"."is-3-widescreen"."is-2-fullhd" {
                                (VehicleCard::new(&TANKOPEDIA[tank_id]).title_style("is-6").predicted_rating(*rating))
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
