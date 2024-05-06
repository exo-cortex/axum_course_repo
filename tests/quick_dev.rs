#![allow(unused)]

use anyhow::Result;
use axum::Json;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> httpc_test::Result<()> {
    let hc = httpc_test::new_client("http://127.0.0.1:8080")?;

    hc.do_get("/hello2/Mike").await?.print().await?; // this one has no cookies

    // hc.do_get("/html/index.html").await?.print().await?;

    let something = hc.do_post("/postaddress/", "something");
    something.await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome"
        }),
    );
    req_login.await?.print().await?;

    hc.do_get("/hello2/Mike").await?.print().await?; // this one has cookies

    Ok(())
}
