use std::env;
use sqlx::PgPool;
use std::pin::Pin;
use std::future::Future;
use tide::{Request, Next};
use crate::{response, Token};
use jsonwebtoken::{decode, Validation, DecodingKey};

pub fn user_token<'a>(
    mut request: Request<PgPool>,
    next:        Next<'a, PgPool>,
) -> Pin<Box<dyn Future<Output = tide::Result> + Send + 'a>> {
    Box::pin(async {
        if request.url().path() == "/" || request.url().path().starts_with("/register") || request.url().path().starts_with("/login") {
            return Ok(next.run(request).await);
        }

        let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
        let token = request.cookie("auth_jwt_secret");
        let validation = Validation::default();

        match token {
            Some(token) => {
                match decode::<Token>(
                    &token.value(),
                    &DecodingKey::from_secret(jwt_secret.as_ref()),
                    &validation,
                ) {
                    Ok(token_data) => {
                        let method = request.method().to_string();
                        
                        if token_data.claims.role == "user" && request.url().path().starts_with("/user") {
                            return response("Forbidden", "Admin only!")
                        }

                        if token_data.claims.role == "user" && method == "POST".to_string() {
                            return response("Forbiden", "You Don't Have Permission")    
                        }
                        
                        if token_data.claims.role == "user" && method == "PUT".to_string() {
                            return response("Forbiden", "You Don't Have Permission")
                        }
                        
                        if token_data.claims.role == "user" && method == "DELETE".to_string() {
                            return response("Forbiden", "You Don't Have Permission")
                        }

                        request.set_ext(token_data);

                        return Ok(next.run(request).await);
                    }
                    Err(err) => {
                        eprintln!("Token Decode Error: {:?}", err);

                        return response("Unauthorized", "Please login first")
                    }
                }
            }
            None => {
                eprintln!("Token Not Found");

                return response("Unauthorized", "Please login first")
            }
        }
    })
}
