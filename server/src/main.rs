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
mod asyncgql;

use actix_web::{guard, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use actix_web_actors::ws;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::Schema;
use async_graphql_actix_web::{GQLRequest, GQLResponse, WSSubscription};
use actix_cors::Cors;
use asyncgql::{BooksSchema, MutationRoot, QueryRoot, Storage, SubscriptionRoot};
use clap::{Arg, App as ClapApp, SubCommand};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let matches = ClapApp::new("yolotrader server")
        .version("1.0")
        .author("Eric Semeniuc <eric.semeniuc@gmail.com>")
        .about("Backend server for yolotrader")
        .arg(Arg::with_name("ip")
            .long("ip")
            .value_name("IP_ADDRESS")
            .help("Listens on the provided interface"))
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .value_name("PORT")
            .help("Listens on the provided port"))
        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let ip = matches.value_of("ip").unwrap_or("0.0.0.0");
    let port = matches.value_of("port").unwrap_or("8080");
    let ip_port = format!("{}:{}", ip, port);

    let pool = db::establish_connection();
    db::run_migrations(&pool.get().unwrap()).expect("Unable to run migrations");

    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(Storage::default())
        .finish();

    println!("Playground: http://{}/graphql", ip_port);

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(web::resource("/").guard(guard::Post()).to(handler::index))
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .guard(guard::Header("upgrade", "websocket"))
                    .to(handler::index_ws),
            )
            .service(web::resource("/").guard(guard::Get()).to(handler::index_playground))
    })
        .bind(ip_port)?
        .run()
        .await


    // HttpServer::new(move || {
    //
    //     App::new()
    //         .wrap(cors_rules)
    //         .data(schema.clone())
    //         .data(pool.clone())
    //         .route("/event", web::post().to(handler::event))
    //         .route("/graphql", web::post().to(handler::graphql))
    //         .route("/graphiql", web::get().to(handler::graphiql))
    //         .route("/", web::get().to(handler::index))
    //         .route("/{_:.*}", web::get().to(handler::dist))
    // })
    //     .bind(listener)?
    //     .run()
    //     .await
}