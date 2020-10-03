#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use actix::prelude::*;
use actix_cors::Cors;
use actix_web::{guard, web, App, HttpServer};
use async_graphql::Schema;
use clap::{App as ClapApp, Arg};
use log::{error, info, trace, warn};
use serde::{Deserialize, Serialize};

use asyncgql::{MutationRoot, QueryRoot, SubscriptionRoot};

mod asyncgql;
mod auth;
mod background_tasks;
mod db;
mod handler;
mod models;
mod push_notification;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    // std::env::set_var("RUST_LOG", "actix_web=info");
    info!("Starting yolotrader with args: {:?}", std::env::args());

    let matches = ClapApp::new("yolotrader server")
        .version("1.0")
        .author("Eric Semeniuc <eric.semeniuc@gmail.com>")
        .about("Backend server for yolotrader")
        .arg(
            Arg::with_name("database_url")
                .long("database_url")
                .value_name("DATABASE_URL")
                .help("The SQLite database file to use, eg. \"db.sqlite\"."),
        )
        .arg(
            Arg::with_name("ip")
                .long("ip")
                .value_name("IP_ADDRESS")
                .help("Listens on the provided interface"),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("PORT")
                .help("Listens on the provided port"),
        )
        .get_matches();

    let database_url = matches.value_of("database_url").unwrap_or("db.sqlite");
    let ip = matches.value_of("ip").unwrap_or("0.0.0.0");
    let port = matches.value_of("port").unwrap_or("8080");
    let ip_port = format!("{}:{}", ip, port);
    info!("Playground available at: http://{}/graphiql", ip_port);

    let pool = db::establish_connection(database_url);
    db::run_migrations(&pool.get().unwrap()).expect("Unable to run migrations");
    db::seed(&pool.get().unwrap()).expect("Unable to seed the database");

    background_tasks::MyActor { pool: pool.clone() }.start();

    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(pool.clone())
        .finish();

    HttpServer::new(move || {
        let cors_rules = if cfg!(debug_assertions) {
            Cors::default()
        } else {
            Cors::new().allowed_methods(vec!["GET", "POST"]).finish()
        };
        App::new()
            .wrap(cors_rules)
            .data(schema.clone())
            .service(
                web::resource("/graphql")
                    .guard(guard::Post())
                    .to(handler::graphql),
            )
            .service(
                web::resource("/graphql")
                    .guard(guard::Get())
                    .guard(guard::Header("upgrade", "websocket"))
                    .to(handler::index_ws),
            )
            .service(
                web::resource("/graphiql")
                    .guard(guard::Get())
                    .to(handler::index_playground),
            )
            .route("/", web::get().to(handler::index))
            .route("/{_:.*}", web::get().to(handler::dist))
    })
    .bind(ip_port)?
    .run()
    .await
}
