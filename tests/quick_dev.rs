use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    let req_list_tickets = hc.do_get("/index.html");
    req_list_tickets.await?.print().await?;
    Ok(())
}
