use sqlx::PgPool;
use tide::{Request, Response};
use serde::{Deserialize, Serialize};
use bcrypt::{hash, verify, DEFAULT_COST};
use std::{env, time::{SystemTime, UNIX_EPOCH}};
use jsonwebtoken::{encode, Header, EncodingKey};

use crate::helper::{response::*, parse::convert_vec_to_values};

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
pub struct TokenStruct {
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
        Err(sqlx::Error::RowNotFound) => {return response_json("error".to_string(), "username not found".to_string(), vec![])}
        Err(err) => {
            eprintln!("Error login: {:?}", err);

            return response_json("error".to_string(), "something went wrong".to_string(), vec![])
        },
    };

    let password_match = verify(body.password.clone(), &user.password).unwrap_or(false);

    if !password_match {
        return response_json("error".to_string(), "password not match".to_string(), vec![])
    }

    let claims = TokenStruct {
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

    let jwt_secret = env::var("JWT_SECRET").unwrap_or(String::from("secret"));
    let key = EncodingKey::from_secret(jwt_secret.as_ref());
    let token = encode(&Header::default(), &claims, &key)?;

    let detail_user = convert_vec_to_values(vec![
        DetailUser {
            id: user.id,
            username: user.username,
            role: user.role.clone(),
        }
    ]);

    response_cookie_json("success".to_string(), "login success".to_string(), detail_user, "set".to_string(), "auth_jwt_secret".to_string(), token.clone())
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
        return response_json("error".to_string(), "username already exists".to_string(), vec![]);
    }

    let hashed_password = hash(body.password, DEFAULT_COST)?;

    match sqlx::query!(
        "insert into client (username, password, role) values ($1, $2, $3)",
        body.username, hashed_password, body.role
    ).execute(pool).await {
        Ok(_) => {response_json("success".to_string(), "signup success".to_string(), vec![])},
        Err(e) => {
            eprintln!("Error register: {}", e);

            response_json("error".to_string(), "failed to signup".to_string(), vec![])
        }
    }
}

pub async fn logout(_req: Request<PgPool>) -> tide::Result<Response> {
    response_cookie_json("success".to_string(), "logout success".to_string(), vec![], "delete".to_string(), "auth_jwt_secret".to_string(), String::from(""))
}