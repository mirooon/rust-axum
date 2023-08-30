use crate::model::{ModelController, Ticket, TicketForCreate};
use crate::Result;
use axum::extract::Path;
use axum::routing::{delete, get, post};
use axum::Router;
use axum::{extract::State, Json};

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(mc)
}

// region: --- REST Handlers
async fn create_ticket(
    State(mc): State<ModelController>,
    Json(ticker_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    let ticket = mc.create_ticket(ticker_fc).await?;
    Ok(Json(ticket))
}

async fn list_tickets(State(mc): State<ModelController>) -> Result<Json<Vec<Ticket>>> {
    let tickets = mc.list_tickets().await?;
    Ok(Json(tickets))
}

async fn delete_ticket(State(mc): State<ModelController>, id: Path<u64>) -> Result<Json<Ticket>> {
    let ticket = mc.delete_ticket(*id).await?;
    Ok(Json(ticket))
}
// endregion: --- REST Handlers
