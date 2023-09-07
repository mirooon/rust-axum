use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::Error;
use crate::Result;
use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::extract::State;
use axum::http::request::Parts;
use axum::{http::Request, middleware::Next, response::Response};
use lazy_regex::regex_captures;
use tower_cookies::Cookie;
use tower_cookies::Cookies;

use crate::web::AUTH_TOKEN;

pub async fn mw_require_auth<B>(
    ctx: Result<Ctx>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    ctx?;
    Ok(next.run(req).await)
}
