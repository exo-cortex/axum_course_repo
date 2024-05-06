use axum::{
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service, post},
    Router,
};
use serde::Deserialize;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

pub use self::error::{Error, Result};
mod error;
mod model;
mod web;

use crate::model::ModelController;

#[tokio::main]
async fn main() -> Result<()> {
    // let example_file = File::open("html/index.html").expect("file could not be opened.");

    let mc = ModelController::new().await?;

    let routes_hello = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", web::routes_tickets::routes(mc.clone())) // clone (Arc) because in case of later
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    // region:    ---Start Server
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    println!(">> LISTENING on {:?}\n", &listener.local_addr());
    axum::serve(listener, routes_hello.into_make_service())
        .await
        .unwrap();
    // endregion: ---Start Server

    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    println!();
    res
}

// region: -- Fallback --
fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./html/")))
}
// endregion: -- Fallback --

// region: --- Routes Hello ---

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:>12} - handler_hello", "Handler");

    let name = params.name.as_deref().unwrap_or("World!");

    Html(format!("Hello <strong>{name}!!!</strong>"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:>12} - handler_hello2", "Handler");

    Html(format!("Hello2 <strong>{name}</strong>"))
}

// endregion: ---Routes Hello ---
