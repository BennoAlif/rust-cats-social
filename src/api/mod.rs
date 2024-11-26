use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use sqlx::Pool;
use sqlx::Postgres;

use crate::AppState;

pub mod cats;
pub mod users;

pub fn user_path() -> actix_web::Scope {
    web::scope("user")
        .service(users::register_user)
        .service(users::login_user)
}

pub fn cat_path() -> actix_web::Scope {
    web::scope("cat")
        .service(cats::get_cats)
        .service(cats::create_cat)
        .service(cats::modify_cat)
        .service(cats::remove_cat)
}

pub fn base_path() -> actix_web::Scope {
    web::scope("v1")
        .service(web::resource("/").to(|| async { "Hello, world!" }))
        .service(user_path())
        .service(cat_path())
}

pub async fn run_server(pool: Pool<Postgres>) -> std::io::Result<()> {
    HttpServer::new(move || {
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .service(base_path())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
