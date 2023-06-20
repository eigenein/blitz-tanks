pub mod giveaway;

use clap::{Args, Parser, Subcommand};

use crate::{
    cli::giveaway::GiveawayArgs, db::Db, prelude::*, trainer, trainer::GridSearchArgs, web,
    web::WebArgs,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    #[clap(long, env = "BLITZ_TANKS_SENTRY_DSN")]
    pub sentry_dsn: Option<String>,

    #[clap(
        long,
        env = "BLITZ_TANKS_SENTRY_TRACES_SAMPLE_RATE",
        default_value = "0.1"
    )]
    pub traces_sample_rate: f32,
}

#[derive(Subcommand)]
pub enum Command {
    /// Run the web application.
    Web(WebArgs),

    /// Pick an account for a giveaway.
    Giveaway(GiveawayArgs),

    /// Train many models, cross-validate them, and pick the best one.
    GridSearch(GridSearchArgs),
}

impl Command {
    pub async fn run(self) -> Result {
        match self {
            Self::Web(args) => web::run(args).await,
            Self::Giveaway(args) => giveaway::run(args).await,
            Self::GridSearch(args) => trainer::run_grid_search(args).await,
        }
    }
}

#[derive(Args)]
pub struct DbArgs {
    /// MongoDB database URI.
    #[clap(
        short = 'd',
        long = "db-uri",
        env = "BLITZ_TANKS_DATABASE_URI",
        default_value = "mongodb://localhost/test?connectTimeoutMS=1000"
    )]
    pub uri: String,
}

impl DbArgs {
    pub async fn open(&self) -> Result<Db> {
        let db = mongodb::Client::with_uri_str(&self.uri)
            .await
            .with_context(|| format!("failed to parse MongoDB URI `{}`", self.uri))?
            .default_database()
            .ok_or_else(|| anyhow!("no default database was specified"))?;
        Ok(db.into())
    }
}
