mod item_item;
mod merge_join_by;

use crate::{
    cli::TrainerArgs,
    prelude::*,
    trainer::item_item::{FitParams, ModelFitter},
};

pub async fn run(args: TrainerArgs) -> Result {
    let votes = args.db.open().await?.votes().await?;
    let model = ModelFitter::new(votes, FitParams::default()).fit().await?;
    Ok(())
}
