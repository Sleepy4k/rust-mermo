use sqlx::PgPool;
use tide::{Request, Next};
use std::{env, pin::Pin, future::Future};
use jsonwebtoken::{decode, Validation, DecodingKey};

use crate::{helper::response::*, controller::auth::TokenStruct};

pub fn user_token<'a>(
    mut request: Request<PgPool>,
    next:        Next<'a, PgPool>,
) -> Pin<Box<dyn Future<Output = tide::Result> + Send + 'a>> {
    Box::pin(async {
        let path = request.url().path().to_string();

        if path == "/" || path == "/register" || path == "/login" {
            return Ok(next.run(request).await);
        }

        let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
        let token = request.cookie("auth_jwt_secret");
        let validation = Validation::default();

        match token {
            Some(token) => {
                match decode::<TokenStruct>(
                    &token.value(),
                    &DecodingKey::from_secret(jwt_secret.as_ref()),
                    &validation,
                ) {
                    Ok(token_data) => {
                        let method = request.method().to_string();
                        
                        if token_data.claims.role == "user" && path.starts_with("/user") {
                            return response_json("forbidden".to_string(), "this route only for admins".to_string(), vec![])
                        }

                        if token_data.claims.role == "user" && method == "POST".to_string() && path != "/logout" {
                            return response_json("forbidden".to_string(), "you don't have permission for this action".to_string(), vec![])
                        }
                        
                        if token_data.claims.role == "user" && method == "PUT".to_string() {
                            return response_json("forbidden".to_string(), "you don't have permission for this action".to_string(), vec![])
                        }
                        
                        if token_data.claims.role == "user" && method == "DELETE".to_string() {
                            return response_json("forbidden".to_string(), "you don't have permission for this action".to_string(), vec![])
                        }

                        request.set_ext(token_data);

                        return Ok(next.run(request).await);
                    }
                    Err(err) => {
                        eprintln!("Token Decode Error: {:?}", err);

                        return response_json("unauthorized".to_string(), "please authorize your self as user".to_string(), vec![])
                    }
                }
            }
            None => {
                eprintln!("Token Not Found");

                return response_json("unauthorized".to_string(), "please authorize your self as user".to_string(), vec![])
            }
        }
    })
}