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
use sentry::integrations::anyhow::capture_anyhow;

use crate::{
    cli::{Cli, Command},
    prelude::*,
};

#[tokio::main]
async fn main() -> Result {
    let args = Cli::parse();
    let _sentry_guard = tracing::init(args.sentry_dsn, 1.0)?;

    let result = match args.command {
        Command::Web(args) => web::run(args).await,
    };

    if let Err(error) = &result {
        capture_anyhow(error);
    }
    result
}
