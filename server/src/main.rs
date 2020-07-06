mod push_notification;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate juniper;

mod asyncgql;
mod auth;
mod db;
mod graphql;
mod handler;
mod models;

use actix_cors::Cors;
use actix_web::client::Client;
use actix_web::{guard, web, App, HttpServer, Result};
use async_graphql::Schema;
use asyncgql::{MutationRoot, QueryRoot, Storage, SubscriptionRoot};
use clap::{App as ClapApp, Arg};
use log::{error, info, trace, warn};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Person {
    latest_price: f64,
    latest_volume: u64,
    latest_update: u64,
}

async fn getter_wrap(tickers: Vec<String>) {
    match getter(tickers).await {
        Ok(_) => trace!("getter() completed successfully"),
        Err(e) => error!("getter() failed with error {}", e),
    };
}

async fn getter(tickers: Vec<String>) -> Result<(), actix_web::Error> {
    // std::env::set_var("RUST_LOG", "actix_http=trace");
    trace!("getting updates");
    trace!(
        "start {}",
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );
    let client = Client::default();
    // Create request builder and send request
    let mut response = client
        .get("https://sandbox.iexapis.com/stable/stock/twtr/quote?filter=latestPrice,latestVolume,latestUpdate&token=Tsk_2311e67e08f1404498c7a7fb91685839") // <--- notice the "s" in "https://..."
        .send()
        .await?; // <- Send http request

    let body = response.body().await?;
    let p: Person = serde_json::from_slice(body.as_ref())?;
    trace!("Downloaded: {:?} ", p);
    trace!(
        "after {}",
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );
    Ok(())
}

use crate::push_notification::send_it;
use actix::prelude::*;
use std::time::Duration;
use std::time::SystemTime;

struct MyActor;

impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(Duration::from_secs(1), move |_this, ctx| {
            // actix_rt::spawn(foo(ii));
            actix_rt::spawn(getter_wrap(vec!["A".to_string()]));
            // ctx.spawn(actix::fut::wrap_future(getter(&vec!["A".to_string()])));
        });
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // let now_future = actix::clock::delay_for(Duration::from_secs(5));
    std::env::set_var("RUST_LOG", "actix_web=info");
    // let a = getter(&vec!["AAPL".to_string(), "GOOG".to_string()]).await;
    MyActor.start();
    let matches = ClapApp::new("yolotrader server")
        .version("1.0")
        .author("Eric Semeniuc <eric.semeniuc@gmail.com>")
        .about("Backend server for yolotrader")
        .arg(
            Arg::with_name("database_url")
                .long("database_url")
                .value_name("DATABASE_URL")
                .help("The SQLite database file to use"),
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

    let pool = db::establish_connection(database_url);
    db::run_migrations(&pool.get().unwrap()).expect("Unable to run migrations");

    // actix_rt::spawn(async move { MyActor.start(); }); //start background fetcher

    // let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(Storage::default())
        .finish();

    println!("Playground: http://{}/graphiql", ip_port);

    HttpServer::new(move || {
        let cors_rules = if cfg!(debug_assertions) {
            Cors::default()
        } else {
            Cors::new()
                .allowed_origin("https://yolotrader.com")
                .allowed_methods(vec!["GET", "POST"])
                .finish()
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
