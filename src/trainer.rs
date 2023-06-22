pub mod item_item;
mod metrics;
mod validate;

use clap::{Args, Subcommand};
use itertools::iproduct;

use crate::{
    cli::DbArgs,
    prelude::*,
    tracing::report_memory_usage,
    trainer::{item_item::Params, validate::search},
};

#[derive(Subcommand)]
pub enum Trainer {
    /// Train many models, cross-validate them, and pick the best one.
    GridSearch(GridSearch),

    /// Fit the model and store it to the database.
    Fit(Fit),
}

impl Trainer {
    pub async fn run(self) -> Result {
        match self {
            Self::GridSearch(grid_search) => grid_search.run().await,
            Self::Fit(fit) => fit.run().await,
        }
    }
}

#[derive(Args)]
pub struct GridSearch {
    #[clap(flatten)]
    db: DbArgs,

    /// Proportion of votes to leave out as a test set.
    #[clap(
        long,
        default_value = "0.2",
        env = "BLITZ_TANKS_TRAINER_TEST_PROPORTION"
    )]
    test_proportion: f64,

    #[clap(long, default_value = "50", env = "BLITZ_TANKS_TRAINER_PARTITIONS")]
    n_partitions: usize,

    /// High `n_neighbors` for grid search.
    #[clap(long, default_value = "15")]
    high_neighbors: usize,
}

impl GridSearch {
    pub async fn run(self) -> Result {
        info!("⏳ Reading the votes…");
        let mut votes = self.db.open().await?.votes().await?.retrieve_all().await?;
        info!(n_votes = votes.len(), "✅ Gotcha!");
        report_memory_usage();

        let params = iproduct!(1..=self.high_neighbors, [false, true], [false, true]).map(
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

#[derive(Args)]
pub struct Fit {
    #[clap(flatten)]
    db: DbArgs,

    #[clap(flatten)]
    model_params: Params,
}

impl Fit {
    pub async fn run(self) -> Result {
        let db = self.db.open().await?;

        let model = {
            info!("⏳ Reading the votes…");
            let votes = db.votes().await?.retrieve_all().await?;
            info!(n_votes = votes.len(), "✅ Gotcha!");
            report_memory_usage();

            info!("⏳ Fitting…");
            self.model_params.fit(&votes)
        };
        info!("✅ Gotcha!");
        report_memory_usage();

        let model_size = mongodb::bson::to_vec(&model)?.len();
        info!(model_size, "📦 Checked the document size");

        let model_id = db.models().await?.insert(&model).await?;
        info!(%model_id, "✅ Saved to the database");

        Ok(())
    }
}
