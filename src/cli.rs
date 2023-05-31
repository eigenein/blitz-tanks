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
        env = "BLITZ_TANKS_DOMAIN_NAME",
        default_value = "localhost:8080"
    )]
    pub domain_name: String,

    #[clap(flatten)]
    pub db: DbArgs,
}

#[derive(Args)]
pub struct WargamingArgs {
    #[clap(short = 'a', long, env = "BLITZ_TANKS_APPLICATION_ID")]
    pub application_id: String,
}

#[derive(Args)]
pub struct DbArgs {
    #[clap(
        short = 'd',
        long = "db-path",
        env = "BLITZ_TANKS_DB_PATH",
        default_value = "blitz-tanks.sled"
    )]
    pub path: PathBuf,
}

impl DbArgs {
    pub fn open(&self) -> Result<Db> {
        Db::open(&self.path)
    }
}
