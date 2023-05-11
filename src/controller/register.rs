use sqlx::PgPool;
use crate::response;
use tide::{Request, Response};
use bcrypt::{hash, DEFAULT_COST};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct SignupRequest {
    username: String,
    password: String,
    role: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct DetailUser {
    id: i32,
    username: String,
    role: String,
}

pub async fn register(mut req: Request<PgPool>) -> tide::Result<Response> {
    let body: SignupRequest = req.body_json().await?;
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
        "INSERT INTO client (username, password, role) VALUES ($1, $2, $3)",
        body.username,
        hashed_password,
        body.role
    ).execute(pool).await {
        Ok(_) => {
            return response("OK", "berhasil signup");
        },
        Err(e) => {
            eprintln!("Error register: {}", e);

            return response("ERROR", "failed to signup");
        }
    }
}
