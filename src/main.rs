mod cli;
mod prelude;

use axum::routing::get;
use axum::Router;
use clap::Parser;

use crate::cli::{Cli, Command};
use crate::prelude::*;

#[tokio::main]
async fn main() -> Result {
    let args = Cli::parse();

    match args.command {
        Command::Web(args) => {
            let app = Router::new().route("/", get(|| async { "Hello, World!" }));
            axum::Server::bind(&args.bind_endpoint)
                .serve(app.into_make_service())
                .await
                .context("the server has failed")
        }
    }
}
