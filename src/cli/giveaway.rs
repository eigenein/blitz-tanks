use std::collections::HashSet;

use futures::TryStreamExt;

use crate::{cli::GiveawayArgs, prelude::*};

pub async fn run(args: GiveawayArgs) -> Result {
    let db = args.db.open().await?;

    info!("⏳ Reading votes…");
    let mut account_ids = db
        .vote_manager()
        .await?
        .iter_all()
        .await?
        .map_ok(|vote| vote.account_id)
        .try_collect::<HashSet<u32>>()
        .await?;

    info!(n_accounts = account_ids.len(), "✅ Votes processed");

    for account_id in args.exclude_ids {
        info!(account_id, "🗑️ Removing excluded account");
        account_ids.remove(&account_id);
    }
    info!(n_accounts = account_ids.len(), "✅ Ready to pick");

    if args.trace_candidates {
        for account_id in &account_ids {
            info!(account_id, "🤞 Candidate");
        }
    }

    let winner_id = fastrand::choice(&account_ids);
    info!(?winner_id, "🎉 Picked a winner!");

    Ok(())
}
