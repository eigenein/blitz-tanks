use std::collections::HashSet;

use clap::Args;
use futures::TryStreamExt;

use crate::{cli::DbArgs, prelude::*};

#[derive(Args)]
pub struct Giveaway {
    #[clap(flatten)]
    db: DbArgs,

    /// Account IDs to include, comma-separated. Ignored, if empty.
    #[clap(long, value_parser, num_args = 0.., value_delimiter = ',')]
    include_ids: Vec<u32>,

    /// Account IDs to exclude, comma-separated.
    #[clap(long, value_parser, num_args = 0.., value_delimiter = ',')]
    exclude_ids: Vec<u32>,

    /// Trace all candidate IDs.
    #[clap(long)]
    trace_candidates: bool,
}

impl Giveaway {
    pub async fn run(self) -> Result {
        let db = self.db.open().await?;

        info!("⏳ Reading votes…");
        let mut account_ids = db
            .votes()
            .await?
            .iter_all()
            .await?
            .map_ok(|vote| vote.id.account_id)
            .try_collect::<HashSet<u32>>()
            .await?;
        info!(n_accounts = account_ids.len(), "✅ Accounts collected");

        if !self.include_ids.is_empty() {
            let include_ids = self.include_ids.into_iter().collect();
            account_ids = account_ids.intersection(&include_ids).copied().collect();
            info!(n_accounts = account_ids.len(), "🚦 Filtered by included IDs");
        }

        for account_id in self.exclude_ids {
            info!(account_id, "🗑️ Removing excluded account");
            account_ids.remove(&account_id);
        }
        info!(n_accounts = account_ids.len(), "✅ Ready to pick");

        if self.trace_candidates {
            for account_id in &account_ids {
                info!(account_id, "🤞 Candidate");
            }
        }

        let winner_id = fastrand::choice(&account_ids);
        info!(?winner_id, "🎉 Picked a winner!");

        Ok(())
    }
}
