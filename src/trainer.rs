mod item_item;
mod metrics;
mod prediction;
mod validate;

use clap::{Args, Subcommand};
use futures::TryStreamExt;
use itertools::iproduct;

use crate::{
    cli::DbArgs,
    models::vote::Vote,
    prelude::*,
    tracing::report_memory_usage,
    trainer::{item_item::Params, validate::search},
};

#[derive(Subcommand)]
pub enum Trainer {
    /// Train many models, cross-validate them, and pick the best one.
    GridSearch(GridSearch),
}

impl Trainer {
    pub async fn run(self) -> Result {
        match self {
            Self::GridSearch(grid_search) => grid_search.run().await,
        }
    }
}

#[derive(Args)]
pub struct GridSearch {
    #[clap(flatten)]
    db: DbArgs,

    #[clap(
        long,
        default_value = "0.2",
        env = "BLITZ_TANKS_TRAINER_TEST_PROPORTION"
    )]
    test_proportion: f64,

    #[clap(long, default_value = "50", env = "BLITZ_TANKS_TRAINER_PARTITIONS")]
    n_partitions: usize,
}

impl GridSearch {
    pub async fn run(self) -> Result {
        info!("⏳ Reading the votes…");
        let mut votes: Vec<Vote> =
            self.db.open().await?.votes().await?.iter_all().await?.try_collect().await?;
        info!(n_votes = votes.len(), "✅ Gotcha!");
        report_memory_usage();

        let params = iproduct!(1..50, [false, true], [false, true]).map(
            |(n_neighbors, enable_damping, include_negative)| Params {
                enable_damping,
                n_neighbors,
                include_negative,
            },
        );
        search(&mut votes, self.n_partitions, self.test_proportion, params);
        info!("🏁 Finished search");

        Ok(())
    }
}
