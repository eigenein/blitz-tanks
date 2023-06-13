pub mod export;
pub mod giveaway;

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

    /// Export all the votes in JSONL format.
    ExportVotes(ExportVotesArgs),

    /// Pick an account for a giveaway.
    Giveaway(GiveawayArgs),
}

#[derive(Args)]
pub struct WebArgs {
    /// Web application bind endpoint.
    #[clap(
        long,
        env = "BLITZ_TANKS_BIND_ENDPOINT",
        default_value = "127.0.0.1:8080"
    )]
    pub bind_endpoint: SocketAddr,

    #[clap(flatten)]
    pub wargaming: WargamingArgs,

    /// Public address used in the hyperlinks.
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
    /// Wargaming.net application ID for the front-end app.
    #[clap(long = "frontend-app-id", env = "BLITZ_TANKS_FRONTEND_APPLICATION_ID")]
    pub frontend_application_id: String,

    /// Wargaming.net application ID for the back-end app.
    #[clap(long = "backend-app-id", env = "BLITZ_TANKS_BACKEND_APPLICATION_ID")]
    pub backend_application_id: String,
}

#[derive(Args)]
pub struct DbArgs {
    /// Legacy Sled database path.
    #[clap(
        short = 'd',
        long = "db-path",
        env = "BLITZ_TANKS_DATABASE_PATH",
        default_value = "db.sled"
    )]
    pub path: PathBuf,

    /// MongoDB database URI.
    #[clap(
        long = "db-uri",
        env = "BLITZ_TANKS_DATABASE_URI",
        default_value = "mongodb://localhost/test"
    )]
    pub uri: String,
}

impl DbArgs {
    pub fn open(&self) -> Result<Db> {
        sled::open(&self.path)
            .with_context(|| format!("failed to open the database from `{:?}`", self.path))
            .map(Db::new)
    }
}

#[derive(Args)]
pub struct ExportVotesArgs {
    #[clap(flatten)]
    pub db: DbArgs,
}

#[derive(Args)]
pub struct GiveawayArgs {
    #[clap(flatten)]
    pub db: DbArgs,

    /// Account IDs to exclude, comma-separated.
    #[clap(long, value_parser, num_args = 0.., value_delimiter = ',')]
    pub exclude_ids: Vec<u32>,

    /// Trace all candidate IDs.
    #[clap(long)]
    pub trace_candidates: bool,
}
