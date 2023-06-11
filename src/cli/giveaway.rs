use std::collections::HashSet;

use tracing::info;

use crate::{cli::GiveawayArgs, prelude::*};

pub fn run(args: GiveawayArgs) -> Result {
    let manager = args.db.open()?;

    info!("⏳ Reading votes…");
    let mut account_ids = manager
        .vote_manager()?
        .iter_all()
        .map(|result| {
            let (account_id, ..) = result?;
            Ok(account_id)
        })
        .collect::<Result<HashSet<u32>>>()?;

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
