#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
extern crate strum;

mod schema;
mod controllers;
mod models;
mod routes;

use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::middleware::Logger;
use diesel::r2d2;
use dotenv::dotenv;
use maplit::hashmap;

pub type DBConPool = r2d2::Pool<r2d2::ConnectionManager<diesel::MysqlConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    dotenv().ok();
    env_logger::init();

    HttpServer::new(|| {
        let database_url = std::env::var("DATABASE_URL").expect("invalid DATABASE_URL");

        App::new()
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allowed_origin(std::env::var("FRONTEND_URL").expect("invalid FRONTEND_URL").as_str())
            )
            .data(
                r2d2::Pool::builder()
                    .build(r2d2::ConnectionManager::<diesel::MysqlConnection>::new(database_url))
                    .expect("Failed to establish DB connection")
            )
            .configure(routes::users)
            .configure(routes::posts)
            .default_service(
                web::route()
                    .to(|| HttpResponse::NotFound().json(
                        hashmap! { "error" => models::error::Error::new(models::error::ErrorCode::InvalidRequest, "No endpoint found.") }
                    ))
            )
    })
        .bind("0.0.0.0:80")?
        .run()
        .await
}
