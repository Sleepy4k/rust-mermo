use sqlx::PgPool;
use crate::response;
use tide::{Request, Response};

#[doc = "function to send response on the root path"]
pub async fn welcome(_req: Request<PgPool>) -> tide::Result<Response> {
    response("OK", "Welcome to the API")
}
