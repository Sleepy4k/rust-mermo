use sqlx::PgPool;
use tide::{Request, Response};

use crate::helper::response::*;

#[doc = "function to send response on the root path"]
pub async fn welcome(_req: Request<PgPool>) -> tide::Result<Response> {
    response_json("success".to_string(), "welcome to the API".to_string(), vec![])
}
