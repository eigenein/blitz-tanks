use serde_json::json;

use crate::{cli::ExportVotes, prelude::*};

pub async fn export_votes(args: ExportVotes) -> Result {
    let manager = args.db.open()?.vote_manager()?;
    for result in manager.iter_all() {
        let (account_id, tank_id, vote) = result?;
        println!("{}", json!({ "account_id": account_id, "tank_id": tank_id, "vote": vote }));
    }
    Ok(())
}
