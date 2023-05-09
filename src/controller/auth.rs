use std::env;
use sqlx::PgPool;
use tide::{Request, Response};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tide::http::cookies::{Cookie, SameSite};
use jsonwebtoken::{encode, Header, EncodingKey};
use crate::{response, response_with_cookie, response_with_data_and_cookie};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct SigninRequest {
    pub username: String,
    pub password: String,
    pub role: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub id: i32,
    pub role: String,
    pub username: String,
    pub iat: u64,
    pub exp: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DetailUser {
    pub id: i32,
    pub username: String,
    pub role: String,
}

pub async fn login(mut req: Request<PgPool>) -> tide::Result<Response> {
    let mut body: SigninRequest = req.body_json().await?;
    let pool = req.state();

    let user = match sqlx::query!("select * from user where username = $1", body.username)
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

    if let Some(hashed_password) = user.password {
        if bcrypt::verify(body.password, &hashed_password).unwrap_or(false) {
            let claims = Token {
                id: user.id.to_string(),
                username: user.username.clone().unwrap(),
                role: user.role.clone().unwrap(),
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

            let mut cookie = Cookie::new("token", token);
            cookie.set_path("/");
            cookie.set_http_only(false);
            cookie.set_same_site(tide::http::cookies::SameSite::Strict);
            // cookie.set_expires(time::OffsetDateTime::now_utc() + time::Duration::days(1));

            response_with_data_and_cookie("OK", "berhasil login", detail_user, "insert", cookie)
        } else {
            response("ERROR", "invalid password")
        }
    } else {
        response("ERROR", "password not found")
    }
}

pub async fn logout(req: Request<PgPool>) -> tide::Result<Response> {
    let cookie = Cookie::build("token", "")
        .http_only(true)
        .same_site(SameSite::Strict)
        .finish();

    response_with_cookie("OK", "berhasil logout", "remove", cookie)
}
