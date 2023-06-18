mod item_item;

use futures::TryStreamExt;

use crate::{
    cli::TrainerArgs,
    models::vote::Vote,
    prelude::*,
    trainer::item_item::{FitParams, ModelFitter},
};

pub async fn run(args: TrainerArgs) -> Result {
    info!("⏳ Reading the votes…");
    let votes: Vec<Vote> =
        args.db.open().await?.votes().await?.iter_all().await?.try_collect().await?;
    info!(n_votes = votes.len(), "✅ Gotcha!");

    let model = ModelFitter::new(&votes, FitParams::default()).fit();
    Ok(())
}
