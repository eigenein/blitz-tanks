use crate::{cli::MigrateVotesArgs, prelude::*};

pub async fn run(args: MigrateVotesArgs) -> Result {
    let manager = args.db.open().await?.vote_manager().await?;
    let mut n_succeeded = 0;
    let mut n_failed = 0;

    for vote in manager.iter_all() {
        let (account_id, tank_id, legacy_vote) = vote?;
        info!(account_id, tank_id, "➡️ Migrating…");
        if let Err(error) = manager.insert_new(account_id, tank_id, &legacy_vote).await {
            error!("❌ Failed to migrate #{account_id}'s vote for vehicle #{tank_id}: {error:#}");
            n_failed += 1;
        } else {
            n_succeeded += 1;
        }
    }

    info!(n_succeeded, n_failed, "☑️ Complete");
    Ok(())
}
