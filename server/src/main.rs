mod push_notification;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;
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
struct IEXPrice {
    latest_price: f64,
    latest_volume: i64,
    latest_update: i64,
}

async fn getter(
    conn: &crate::db::DbPoolConn,
    tickers: Vec<String>,
) -> Result<(), actix_web::Error> {
    // std::env::set_var("RUST_LOG", "actix_http=trace");
    trace!("getting updates from IEX");
    trace!(
        "start {}",
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );

    let client = Client::default();
    // Create request builder and send request
    for ticker in tickers {
        let url = format!("https://sandbox.iexapis.com/stable/stock/{}/quote?filter=latestPrice,latestVolume,latestUpdate&token=Tsk_2311e67e08f1404498c7a7fb91685839", ticker);
        let mut response = client.get(url).send().await?;
        let body = response.body().await?;
        let p: IEXPrice = serde_json::from_slice(body.as_ref())?;
        IntradayPrice::insert(
            conn,
            ticker,
            p.latest_price,
            p.latest_volume,
            chrono::NaiveDateTime::from_timestamp(p.latest_update, 0),
        );
        trace!("Downloaded: {:?} ", p);
    }
    trace!(
        "after {}",
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );
    Ok(())
}

use crate::models::IntradayPrice;
use crate::push_notification::send_it;
use actix::prelude::*;
use std::time::Duration;
use std::time::SystemTime;

struct MyActor {
    pool: db::DbPool,
}

impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(Duration::from_secs(1), move |this, ctx| {
            let conn = match this.pool.get() {
                Ok(v) => v,
                Err(e) => {
                    warn!("Failed to get pool conn connection");
                    return;
                }
            };

            ctx.spawn(actix::fut::wrap_future(async move {
                //spawn a separate task since we don't want to block based on prev request
                let result = getter(&conn, vec!["A".to_string()]).await;
                result.map_err(|err| warn!("Failed to get data from iex, {}", err));
            }));
        });
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // let now_future = actix::clock::delay_for(Duration::from_secs(5));
    std::env::set_var("RUST_LOG", "actix_web=info");
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

    let pool = db::establish_connection(database_url);
    db::run_migrations(&pool.get().unwrap()).expect("Unable to run migrations");

    MyActor { pool: pool.clone() }.start();

    // let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(pool.clone())
        .finish();

    println!("Playground: http://{}/graphiql", ip_port);

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
