use clap::{Args, Parser, Subcommand};

use crate::{
    db::Db,
    giveaway::Giveaway,
    prelude::*,
    tankopedia::{bundler::BundleTankopedia, unpacker::UnpackData},
    trainer::Trainer,
    web::Web,
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
    Web(Web),

    /// Pick an account for a giveaway.
    Giveaway(Giveaway),

    /// Trainer subcommands.
    #[command(subcommand)]
    Trainer(Trainer),

    /// Parse the client resources and bundle the tankopedia to the source code.
    BundleTankopedia(BundleTankopedia),

    /// Unpack DVPL's recursively in the specified directory.
    UnpackData(UnpackData),
}

impl Command {
    pub async fn run(self) -> Result {
        match self {
            Self::BundleTankopedia(bundle_tankopedia) => bundle_tankopedia.run().await,
            Self::Giveaway(giveaway) => giveaway.run().await,
            Self::Trainer(trainer) => trainer.run().await,
            Self::UnpackData(unpack_data) => unpack_data.run(),
            Self::Web(web) => web.run().await,
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
