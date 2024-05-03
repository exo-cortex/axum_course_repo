use std::os::unix::net::SocketAddr;

use axum::extract::Query;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;
use serde::Deserialize;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route("/hello", get(handler_hello));

    // region:    ---Start Server
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    // info!("{:<12} - {:?}\n", "LISTENING", listener.local_addr());
    println!(">> LISTENING on {:?}\n", &listener.local_addr());
    axum::serve(listener, routes_hello.into_make_service())
        .await
        .unwrap();
    // endregion: ---Start Server
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:>12} - handler_hello", "Handler");

    let name = params.name.as_deref().unwrap_or("World!");

    Html("Hello <strong>{World}!!!</strong>")
}
