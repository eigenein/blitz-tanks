mod item_item;
mod metrics;
mod prediction;
mod validate;

use clap::Args;
use futures::TryStreamExt;
use itertools::iproduct;

use crate::{
    cli::DbArgs,
    models::vote::Vote,
    prelude::*,
    tracing::report_memory_usage,
    trainer::{item_item::Params, validate::search},
};

#[derive(Args)]
pub struct GridSearchArgs {
    #[clap(flatten)]
    pub db: DbArgs,

    #[clap(
        long,
        default_value = "0.2",
        env = "BLITZ_TANKS_TRAINER_TEST_PROPORTION"
    )]
    pub test_proportion: f64,

    #[clap(long, default_value = "50", env = "BLITZ_TANKS_TRAINER_PARTITIONS")]
    pub n_partitions: usize,
}

pub async fn run_grid_search(args: GridSearchArgs) -> Result {
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
    search(&mut votes, args.n_partitions, args.test_proportion, params);
    info!("🏁 Finished search");

    Ok(())
}
