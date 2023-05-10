use sqlx::PgPool;
use crate::response;
use tide::{Request, Response};

pub async fn welcome(_req: Request<PgPool>) -> tide::Result<Response> {
    response("OK", "Welcome to the API")
}