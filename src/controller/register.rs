use sqlx::PgPool;
use crate::response;
use tide::{Request, Response};
use bcrypt::{hash, DEFAULT_COST};

#[doc = "Define the struct for validate request body of \"register\""]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct RegisterRequest {
    username: String,
    password: String,
    role: String,
}

#[doc = "Define the struct for response body of \"register\""]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct DetailUser {
    id: i32,
    username: String,
    role: String,
}

#[doc = "function to register user"]
pub async fn register(mut req: Request<PgPool>) -> tide::Result<Response> {
    let body: RegisterRequest = req.body_json().await?;
    let pool = req.state();

    let user_check = sqlx::query!(
        "SELECT id, username FROM client WHERE username = $1",
        body.username.clone()
    ).fetch_optional(pool).await?;

    if user_check.is_some() {
        return response("ERROR", "username already exists");
    }

    let hashed_password = hash(body.password, DEFAULT_COST)?;

    match sqlx::query!(
        "insert into client (username, password, role) values ($1, $2, $3)",
        body.username, hashed_password, body.role
    ).execute(pool).await {
        Ok(_) => {response("OK", "berhasil signup")},
        Err(e) => {
            eprintln!("Error register: {}", e);

            response("ERROR", "failed to signup")
        }
    }
}
