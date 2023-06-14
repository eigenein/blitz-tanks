use serde_json::json;

use crate::{cli::ExportVotesArgs, prelude::*};

pub async fn export_votes(args: &ExportVotesArgs) -> Result {
    let manager = args.db.open().await?.vote_manager().await?;
    for result in manager.iter_all() {
        let (account_id, tank_id, vote) = result?;
        println!("{}", json!({ "account_id": account_id, "tank_id": tank_id, "vote": vote }));
    }
    Ok(())
}
