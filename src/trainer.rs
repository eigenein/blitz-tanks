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
        validate::fit_and_validate,
    },
};

pub async fn run(args: TrainerArgs) -> Result {
    info!("⏳ Reading the votes…");
    let mut votes: Vec<Vote> =
        args.db.open().await?.votes().await?.iter_all().await?.try_collect().await?;
    info!(n_votes = votes.len(), "✅ Gotcha!");
    report_memory_usage();

    let n_test = votes.len() / 3;
    fastrand::shuffle(&mut votes);
    let (mean_reciprocal_rank,) = fit_and_validate(
        &votes[n_test..],
        &votes[..n_test],
        &FitParams { disable_damping: false },
        &PredictParams {
            n_neighbors: 10,
            include_negative: false,
        },
    );
    info!(mean_reciprocal_rank);

    Ok(())
}
