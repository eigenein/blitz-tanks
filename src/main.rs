mod prelude;

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use axum::routing::get;
use axum::Router;

use crate::prelude::*;

#[tokio::main]
async fn main() -> Result {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    axum::Server::bind(&SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 8080)))
        .serve(app.into_make_service())
        .await
        .context("the server has failed")
}
