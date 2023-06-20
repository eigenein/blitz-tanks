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
mod tracing;
mod trainer;
mod web;
mod wg;

use clap::Parser;

use crate::{cli::Cli, prelude::*, tracing::trace};

#[global_allocator]
static ALLOCATOR: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[tokio::main]
async fn main() -> Result {
    let args = Cli::parse();
    let _sentry_guard = tracing::init(args.sentry_dsn, args.traces_sample_rate)?;
    trace(args.command.run().await)
}
