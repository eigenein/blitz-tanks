mod item_item;
mod validate;

use futures::TryStreamExt;

use crate::{
    cli::TrainerArgs,
    models::vote::Vote,
    prelude::*,
    tracing::report_memory_usage,
    trainer::{
        item_item::{FitParams, PredictParams},
        validate::fit_and_cross_validate,
    },
};

pub async fn run(args: TrainerArgs) -> Result {
    info!("⏳ Reading the votes…");
    let mut votes: Vec<Vote> =
        args.db.open().await?.votes().await?.iter_all().await?.try_collect().await?;
    info!(n_votes = votes.len(), "✅ Gotcha!");
    report_memory_usage();

    fastrand::shuffle(&mut votes);
    let (mean_reciprocal_rank,) = fit_and_cross_validate(
        &mut votes,
        10,
        &FitParams { disable_damping: false },
        &PredictParams {
            n_neighbors: 20,
            include_negative: false,
        },
    );
    info!(mean_reciprocal_rank, "🏁 Finished cross validation");

    Ok(())
}
