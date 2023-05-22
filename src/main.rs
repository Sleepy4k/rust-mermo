use sqlx::PgPool;
use dotenv::dotenv;
use mermo::{path, user_token};
use tide::http::headers::HeaderValue;
use tide::security::{CorsMiddleware, Origin};

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DB config error");

    let pool = PgPool::connect(&db_url).await.expect("DB Connection error");

    let cors = CorsMiddleware::new()
        .allow_methods("GET, POST, PUT, DELETE, OPTIONS".parse::<HeaderValue>().unwrap())
        .allow_origin(Origin::from("http://localhost:3000"))
        .allow_headers("Content-Type".parse::<HeaderValue>().unwrap())
        .allow_credentials(true);

    tide::log::start();

    let mut app = tide::with_state(pool.clone());

    app.with(cors);
    // app.with(user_token);
    
    path(&mut app).await;

    app.listen("0.0.0.0:7004").await?;

    Ok(())
}