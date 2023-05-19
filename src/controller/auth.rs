use std::env;
use sqlx::PgPool;
use tide::{Request, Response};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Header, EncodingKey};
use crate::{response, response_with_cookie, response_with_data_and_cookie};

#[doc = "Define the struct for validate request body of \"login\""]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct LoginRequest {
    username: String,
    password: String
}

#[doc = "Define the struct for validate request body of \"register\""]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct RegisterRequest {
    username: String,
    password: String,
    role: String,
}

#[doc = "Define the struct of the token"]
#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub id: i32,
    pub role: String,
    pub username: String,
    pub iat: u64,
    pub exp: u64,
}

#[doc = "Define the struct of the user"]
#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: i32,
    username: String,
    password: String,
    role: String,
}

#[doc = "Define the struct of the detail user"]
#[derive(Serialize, Deserialize, Debug)]
struct DetailUser {
    id: i32,
    username: String,
    role: String,
    token: String,
}

#[doc = "Define the login function"]
pub async fn login(mut req: Request<PgPool>) -> tide::Result<Response> {
    let body: LoginRequest = req.body_json().await?;
    let pool = req.state();

    let user = match sqlx::query!(
        "select * from client where username = $1",
        body.username
    ).fetch_one(pool).await {
        Ok(user) => user,
        Err(sqlx::Error::RowNotFound) => {
            return response("ERROR", "username not found")
        }
        Err(err) => {
            eprintln!("Error login: {:?}", err);

            return response("Error", "something went wrong")
        },
    };

    let password_match = verify(body.password.clone(), &user.password).unwrap_or(false);

    if password_match {
        let claims = Token {
            id: user.id,
            username: user.username.clone(),
            role: user.role.clone(),
            iat: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            exp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .saturating_add(60 * 60),
        };

        let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
        let key = EncodingKey::from_secret(jwt_secret.as_ref());
        let token = encode(&Header::default(), &claims, &key)?;

        let detail_user = DetailUser {
            id: user.id,
            username: user.username,
            role: user.role.clone(),
            token: token.clone(),
        };

        response_with_data_and_cookie("OK", "berhasil login", vec![detail_user], "insert", "auth_jwt_secret", token.clone())
    } else {
        response("ERROR", "password not match")
    }
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

pub async fn logout(_req: Request<PgPool>) -> tide::Result<Response> {
    response_with_cookie("OK", "berhasil logout", "remove", "auth_jwt_secret", String::from(""))
}