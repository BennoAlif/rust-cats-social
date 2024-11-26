mod api;
mod configs;
mod entities;
mod helpers;
mod middlewares;
mod repositories;

use api::run_server;
use configs::db::create_pool;
use dotenv::dotenv;
use sqlx::{Pool, Postgres};

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let environment = std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string());

    if environment == "production" {
        std::env::set_var("RUST_LOG", "info");
        std::env::set_var("RUST_BACKTRACE", "0");
    } else {
        std::env::set_var("RUST_LOG", "debug");
        std::env::set_var("RUST_BACKTRACE", "1");
    }
    env_logger::init();

    let pool = create_pool().await;
    run_server(pool).await
}
