use crate::Error;
use crate::Result;
use axum::{http::Request, middleware::Next, response::Response};
use tower_cookies::Cookies;

use crate::web::AUTH_TOKEN;

pub async fn mw_require_auth<B>(
    cookies: Cookies,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
    println!("auth token {:?}", auth_token);
    auth_token.ok_or(Error::AuthFailedNoAuthTokenCookie)?;
    Ok(next.run(req).await)
}
