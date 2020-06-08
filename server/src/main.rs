#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate juniper;

mod auth;
mod db;
mod graphql;
mod handler;
mod models;

use actix_web::{web, App, HttpServer};
use actix_cors::Cors;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let listener = std::env::var("IP_PORT").expect("IP_PORT must be set. Eg. 0.0.0.0:80");

    let schema = std::sync::Arc::new(graphql::create_schema());
    let pool = db::establish_connection();
    db::run_migrations(&pool.get().unwrap()).expect("Unable to run migrations");
    HttpServer::new(move || {
        let cors_rules = if cfg!(debug_assertions) {
            Cors::default()
        } else {
            Cors::new()
                .allowed_origin("https://decloak.io")
                .allowed_methods(vec!["GET", "POST"])
                .finish()
        };
        App::new()
            .wrap(cors_rules)
            .data(schema.clone())
            .data(pool.clone())
            .route("/event", web::post().to(handler::event))
            .route("/graphql", web::post().to(handler::graphql))
            .route("/graphiql", web::get().to(handler::graphiql))
            .route("/", web::get().to(handler::index))
            .route("/{_:.*}", web::get().to(handler::dist))
    })
        .bind(listener)?
        .run()
        .await
}