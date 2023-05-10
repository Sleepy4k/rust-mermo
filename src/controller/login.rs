use std::env;
use sqlx::PgPool;
use tide::{Request, Response};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Header, EncodingKey};
use crate::{response, response_with_data_and_cookie};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct SigninRequest {
    username: String,
    password: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Token {
    id: i32,
    role: String,
    username: String,
    iat: u64,
    exp: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: i32,
    username: String,
    password: String,
    role: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct DetailUser {
    id: i32,
    username: String,
    role: String,
}

pub async fn login(mut req: Request<PgPool>) -> tide::Result<Response> {
    let body: SigninRequest = req.body_json().await?;
    let pool = req.state();

    let user = match sqlx::query!("select * from client where username = $1", body.username)
        .fetch_one(pool)
        .await
        {
            Ok(user) => user,
            Err(sqlx::Error::RowNotFound) => {
                return response("ERROR", "username not found")
            }
            Err(err) => {
                eprintln!("Error login: {:?}", err);

                return response("Error", "something went wrong")
            },
        };

    let hashed_password = hash(body.password.clone(), DEFAULT_COST).unwrap();

    let password_match = verify(user.password, &hashed_password).unwrap_or(false);

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
                .saturating_add(60 * 60), // Token is valid for 1 hour
        };

        let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
        let key = EncodingKey::from_secret(jwt_secret.as_ref());
        let token = encode(&Header::default(), &claims, &key)?;

        let detail_user = DetailUser {
            id: user.id,
            username: user.username,
            role: user.role.clone(),
        };

        return response_with_data_and_cookie("OK", "berhasil login", vec![detail_user], "insert", "token", token)
    } else {
        return response("ERROR", "password not match")
    }
}
