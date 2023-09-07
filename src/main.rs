mod config;
mod ctx;
mod error;
mod model;
mod web;

pub use self::error::{Error, Result};
pub use config::config;

use crate::model::ModelManager;
use std::net::SocketAddr;

use axum::{
    extract::{Path, Query},
    http::{Method, Uri},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    Json, Router,
};
use ctx::Ctx;
use serde::Deserialize;
use serde_json::json;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

pub mod _dev_utils;

#[tokio::main]
async fn main() -> Result<()> {
    info!("{:<12} - main", "FOR-DEV-ONLY");

    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // for dev only

    _dev_utils::init_dev().await;
    let mc = ModelManager::new().await?;

    let routes_all = Router::new()
        .merge(routes_hello())
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    // region: --- Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    info!("=>> LISTNINEG on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
    // endregion: --- Start Server
}

async fn main_response_mapper(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    let uuid = Uuid::new_v4();

    // -- Get the eventual response error.
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    // -- If client error, build the new reponse.
    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string(),
                }
            });

            // Build the new response from the client_error_body
            (*status_code, Json(client_error_body)).into_response()
        });

    // Build and log the server log line.
    let client_error = client_status_error.unzip().1;
    // TODO: Need to hander if log_request fail (but should not fail request)
    // let _ = log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

    error_response.unwrap_or(res)
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

// region: --- Handler Hello

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
    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <strong>{name}</strong>"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    Html(format!("Hello <strong>{name}</strong>"))
}
// endregion: --- Handler Hello
