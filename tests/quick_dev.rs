use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    hc.do_get("/hello2/Jan").await?.print().await?;
    hc.do_get("/src/main.rs").await?.print().await?;

    let req_create_ticket = hc.do_post("/api/tickets", json!({"title": "My title"}));
    req_create_ticket.await?.print().await?;

    let req_list_tickets = hc.do_get("/api/tickets");
    req_list_tickets.await?.print().await?;

    let req_delete_ticket = hc.do_delete("/api/tickets/2");
    req_delete_ticket.await?.print().await?;

    let req_login = hc.do_post("/api/login", json!({"username": "demo1", "pwd": "welcome"}));

    req_login.await?.print().await?;

    let req_list_tickets = hc.do_get("/api/tickets");
    req_list_tickets.await?.print().await?;

    Ok(())
}
