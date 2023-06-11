#![warn(
    clippy::all,
    clippy::explicit_into_iter_loop,
    clippy::manual_let_else,
    clippy::map_unwrap_or,
    clippy::missing_const_for_fn,
    clippy::needless_pass_by_value,
    clippy::trivially_copy_pass_by_ref,
    clippy::unused_self
)]

mod cli;
mod db;
mod models;
mod prelude;
mod tracing;
mod web;
mod weegee;

use clap::Parser;
use serde_json::json;

use crate::{
    cli::{Cli, Command, ListVotesArgs},
    prelude::*,
    tracing::trace,
};

#[tokio::main]
async fn main() -> Result {
    let args = Cli::parse();
    let _sentry_guard = tracing::init(args.sentry_dsn, 1.0)?;

    match args.command {
        Command::Web(args) => trace(web::run(args).await),
        Command::ListVotes(args) => trace(list_votes(args).await),
    }
}

async fn list_votes(args: ListVotesArgs) -> Result {
    let manager = args.db.open()?.vote_manager()?;
    for result in manager.iter_all() {
        let (account_id, tank_id, vote) = result?;
        println!("{}", json!({ "account_id": account_id, "tank_id": tank_id, "vote": vote }));
    }
    Ok(())
}
