mod item_item;
mod metrics;
mod prediction;
mod validate;

use futures::TryStreamExt;
use itertools::iproduct;

use crate::{
    cli::TrainerArgs,
    models::vote::Vote,
    prelude::*,
    tracing::report_memory_usage,
    trainer::{item_item::Params, validate::search},
};

pub async fn run(args: TrainerArgs) -> Result {
    info!("⏳ Reading the votes…");
    let mut votes: Vec<Vote> =
        args.db.open().await?.votes().await?.iter_all().await?.try_collect().await?;
    info!(n_votes = votes.len(), "✅ Gotcha!");
    report_memory_usage();

    let params = iproduct!(1..50, [false, true], [false, true]).map(
        |(n_neighbors, disable_damping, include_negative)| Params {
            disable_damping,
            n_neighbors,
            include_negative,
        },
    );
    let (reciprocal_rank, params) =
        search(&mut votes, args.n_partitions, args.test_proportion, params).unwrap();
    info!(
        %reciprocal_rank,
        rank = reciprocal_rank.rank(),
        disable_damping = params.disable_damping,
        n_neighbors = params.n_neighbors,
        include_negative = params.include_negative,
        "🏁 Finished search"
    );

    Ok(())
}
