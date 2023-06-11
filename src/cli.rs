use std::{net::SocketAddr, path::PathBuf};

use clap::{Args, Parser, Subcommand};

use crate::{db::Db, prelude::*};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    #[clap(long, env = "BLITZ_TANKS_SENTRY_DSN")]
    pub sentry_dsn: Option<String>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Run the web application.
    Web(WebArgs),

    /// List all the votes in JSONL format.
    ListVotes(ListVotesArgs),
}

#[derive(Args)]
pub struct WebArgs {
    #[clap(
        long,
        env = "BLITZ_TANKS_BIND_ENDPOINT",
        default_value = "127.0.0.1:8080"
    )]
    pub bind_endpoint: SocketAddr,

    #[clap(flatten)]
    pub wargaming: WargamingArgs,

    #[clap(
        long,
        env = "BLITZ_TANKS_PUBLIC_ADDRESS",
        default_value = "localhost:8080"
    )]
    pub public_address: String,

    #[clap(flatten)]
    pub db: DbArgs,

    /// Update the tankopedia database on startup.
    #[clap(long, env = "BLITZ_TANKS_UPDATE_TANKOPEDIA")]
    pub update_tankopedia: bool,
}

#[derive(Args)]
pub struct WargamingArgs {
    #[clap(long = "frontend-app-id", env = "BLITZ_TANKS_FRONTEND_APPLICATION_ID")]
    pub frontend_application_id: String,

    #[clap(long = "backend-app-id", env = "BLITZ_TANKS_BACKEND_APPLICATION_ID")]
    pub backend_application_id: String,
}

#[derive(Args)]
pub struct DbArgs {
    #[clap(
        short = 'd',
        long = "db-path",
        env = "BLITZ_TANKS_DATABASE_PATH",
        default_value = "db.sled"
    )]
    pub path: PathBuf,
}

impl DbArgs {
    pub fn open(&self) -> Result<Db> {
        Db::open(&self.path)
    }
}

#[derive(Args)]
pub struct ListVotesArgs {
    #[clap(flatten)]
    pub db: DbArgs,
}
