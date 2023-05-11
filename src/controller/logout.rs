use sqlx::PgPool;
use tide::{Request, Response};
use crate::response_with_cookie;

pub async fn logout(_req: Request<PgPool>) -> tide::Result<Response> {
    response_with_cookie("OK", "berhasil logout", "remove", "auth_jwt_secret", String::from(""))
}