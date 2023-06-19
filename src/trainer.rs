mod item_item;

use std::collections::HashMap;

use futures::TryStreamExt;

use crate::{
    cli::TrainerArgs,
    models::{rating::Rating, vote::Vote},
    prelude::*,
    tracing::report_memory_usage,
    trainer::item_item::{FitParams, Model, PredictParams},
};

pub async fn run(args: TrainerArgs) -> Result {
    info!("⏳ Reading the votes…");
    let votes: Vec<Vote> =
        args.db.open().await?.votes().await?.iter_all().await?.try_collect().await?;
    info!(n_votes = votes.len(), "✅ Gotcha!");
    report_memory_usage();

    info!("🔢 Fitting…");
    let model = Model::fit(&votes, &FitParams { disable_damping: true });
    info!("✅ Gotcha!");
    report_memory_usage();

    Ok(())
}
