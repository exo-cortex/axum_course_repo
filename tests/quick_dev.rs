#![allow(unused)]

use anyhow::Result;
use axum::Json;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> httpc_test::Result<()> {
    let hc = httpc_test::new_client("http://127.0.0.1:3000")?;

    hc.do_get("/hello2/Mike").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome"
        }),
    );
    req_login.await?.print().await?;

    Ok(())
}
