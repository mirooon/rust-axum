mod config;
mod crypt;
mod ctx;
mod error;
mod model;
mod utils;
mod web;

pub use self::error::{Error, Result};
pub use config::config;

use crate::{
    model::ModelManager,
    web::{
        mw_auth::{mw_ctx_require, mw_ctx_resolve},
        mw_res_map::mw_response_map,
        rpc,
    },
};
use std::net::SocketAddr;

use crate::web::routes_login;
use axum::{
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse},
    routing::{get, get_service},
    Router,
};
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::EnvFilter;

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
    let mm = ModelManager::new().await?;

    let routes_rpc = rpc::routes(mm.clone()).route_layer(middleware::from_fn(mw_ctx_require));

    let routes_all = Router::new()
        .merge(routes_login::routes(mm.clone()))
        .nest("/api", routes_rpc)
        .layer(middleware::map_response(mw_response_map))
        .layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolve))
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

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

// region: --- Handler Hello

fn routes_hello() -> Router {
    Router::new().route("/hello", get(handler_hello))
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
