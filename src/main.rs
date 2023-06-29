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
mod giveaway;
mod models;
mod prelude;
mod tankopedia;
mod tracing;
mod trainer;
mod web;
mod wg;

use std::env::var;

use clap::Parser;

use crate::{cli::Cli, prelude::*, tracing::trace};

#[global_allocator]
static ALLOCATOR: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[tokio::main]
async fn main() -> Result {
    let envfile_path = var("BLITZ_TANKS_ENVFILE").unwrap_or_else(|_| ".env".to_string());
    let dotenv_result = dotenvy::from_path_override(&envfile_path);
    let cli = Cli::parse();
    let _sentry_guard = tracing::init(cli.sentry_dsn, cli.traces_sample_rate)?;
    if let Err(error) = dotenv_result {
        warn!("⚠️ Failed to load environment from `{envfile_path}`: {error:#}");
    }
    trace(cli.command.run().await)
}
