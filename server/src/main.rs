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
use actix_web::client::Client;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Person {
    latest_price: f64,
    latest_volume: u64,
    latest_update: u64,
}

async fn getter(tickers: &Vec<String>) -> Result<(), actix_web::Error> {
    std::env::set_var("RUST_LOG", "actix_http=trace");
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => println!("start {}", n.as_secs()),
        Err(_) => println!("err"),
    }
    std::thread::sleep(Duration::from_secs(5));

    let client = Client::default();

    // Create request builder and send request
    // let mut response = client
    //     .get("https://sandbox.iexapis.com/stable/stock/twtr/quote?filter=latestPrice,latestVolume,latestUpdate&token=Tsk_2311e67e08f1404498c7a7fb91685839") // <--- notice the "s" in "https://..."
    //     .send()
    //     .await?; // <- Send http request
    // let body = response.body().await?;
    // let p: Person = serde_json::from_slice(body.as_ref())?;
    // println!("Downloaded: {:?} ", p);
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => println!("after {}", n.as_secs()),
        Err(_) => println!("err"),
    }
    Ok(())
}

// fn doit() -> () {
//     let now_future = Delay::new(Duration::from_secs(5));
//
//     actix_rt::spawn(now_future.map(|x| {
//         println!("waited for 5 secs");
//     }));
//     /***************************/
//
// }


use actix::prelude::*;
use std::time::Duration;
use std::time::SystemTime;

struct MyActor;

impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(Duration::from_secs(1),
                         |_this, ctx| {


                             getter(&vec!["A".to_string()]);

                         });
    }
}

fn doit() {
    loop {
        for i in 1..3 {
            actix_rt::spawn(async move { MyActor.start(); }); //start background fetcher
        }
        std::thread::sleep(Duration::from_secs(5));
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let now_future = actix::clock::delay_for(Duration::from_secs(5));
    std::env::set_var("RUST_LOG", "actix_web=info");
    let a = getter(&vec!["AAPL".to_string(), "GOOG".to_string()]).await;

    let matches = ClapApp::new("yolotrader server")
        .version("1.0")
        .author("Eric Semeniuc <eric.semeniuc@gmail.com>")
        .about("Backend server for yolotrader")
        .arg(Arg::with_name("database_url")
            .long("database_url")
            .value_name("DATABASE_URL")
            .help("The SQLite database file to use"))
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

    let database_url = matches.value_of("database_url").unwrap_or("db.sqlite");
    let ip = matches.value_of("ip").unwrap_or("0.0.0.0");
    let port = matches.value_of("port").unwrap_or("8080");
    let ip_port = format!("{}:{}", ip, port);

    let pool = db::establish_connection(database_url);
    db::run_migrations(&pool.get().unwrap()).expect("Unable to run migrations");

    actix_rt::spawn(async move { MyActor.start(); }); //start background fetcher

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
            .service(web::resource("/graphql")
                .guard(guard::Post())
                .to(handler::graphql))
            .service(web::resource("/graphql")
                .guard(guard::Get())
                .guard(guard::Header("upgrade", "websocket"))
                .to(handler::index_ws))
            .service(web::resource("/graphiql").guard(guard::Get()).to(handler::index_playground))
            .route("/", web::get().to(handler::index))
            .route("/{_:.*}", web::get().to(handler::dist))
    })
        .bind(ip_port)?
        .run()
        .await
}