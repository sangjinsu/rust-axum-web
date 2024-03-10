#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn test_quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://127.0.0.1:8080")?;
    hc.do_get("/hello/sangjinsu").
        await?.print().await?;


    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "admin",
            "password": "admin"
        }),
    );

    let res_login = req_login.await?;
    res_login.print().await?;

    Ok(())
}
