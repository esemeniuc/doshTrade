use actix::prelude::*;
use actix_cors::Cors;
use actix_web::{App, guard, HttpServer, web};
use async_graphql::Schema;
use clap::Arg;

use asyncgql::{MutationRoot, QueryRoot, Subscription};
use std::sync::{RwLock, Arc};
use std::collections::HashSet;
use anyhow::Context;

mod asyncgql;
mod auth;
mod background_tasks;
mod db;
mod handler;
mod models;
mod push_notification;

pub type StockPool = Arc<RwLock<HashSet<String>>>;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    // std::env::set_var("RUST_LOG", "actix_web=info");
    // std::env::set_var("DATABASE_URL", "postgresql://postgres:mysecretpassword@localhost"); //for sqlx::query! macro
    log::info!("Starting yolotrader with args: {:?}", std::env::args());

    let matches = clap::App::new("yolotrader server")
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

    let database_url = matches
        .value_of("database_url")
        .unwrap_or("postgresql://postgres:mysecretpassword@localhost");
    let ip = matches.value_of("ip").unwrap_or("0.0.0.0");
    let port = matches.value_of("port").unwrap_or("8080");
    let ip_port = format!("{}:{}", ip, port);
    log::info!("Playground available at: http://{}/graphiql", ip_port);

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(50)
        .connect(database_url)
        .await
        .expect("Unable to connect to database pool");
    // use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
    // use std::str::FromStr;
    // let pool = //sqlx::sqlite::SqlitePool::connect(database_url)
    //     sqlx::sqlite::SqlitePool::connect_with(SqliteConnectOptions::from_str(database_url).unwrap()
    //         .busy_timeout(std::time::Duration::new(5, 0))
    //         .create_if_missing(true))
    //         .await
    //         .expect("Unable to connect to database pool");
    db::seed(&pool).await.expect("Error seeding the database");

    let tickers_to_fetch = crate::models::Stock::get_unique_tickers(&pool)
        .await?
        .into_iter()
        .collect::<HashSet<_>>();
    let stock_list: StockPool = Arc::new(RwLock::new(tickers_to_fetch));

    let schema = Schema::build(QueryRoot, MutationRoot, Subscription)
        .data(pool.clone())
        .data(stock_list.clone())
        .finish();

    let actor = background_tasks::stock_actor::StockActor { pool: pool.clone(), stock_list: stock_list.clone() };
    background_tasks::stock_actor::StockActor::start_in_arbiter(&Arbiter::new(), move |_| actor);

    let actor = background_tasks::options_actor::OptionsActor { pool: pool.clone(), stock_list: stock_list.clone() };
    background_tasks::options_actor::OptionsActor::start_in_arbiter(&Arbiter::new(), move |_| actor);

    let actor = background_tasks::push_notifications_actor::PushNotificationsActor { pool: pool.clone(), stock_list: stock_list.clone() };
    background_tasks::push_notifications_actor::PushNotificationsActor::start_in_arbiter(&Arbiter::new(), move |_| actor);

    HttpServer::new(move || {
        let cors_rules = if cfg!(debug_assertions) {
            Cors::permissive()
        } else {
            Cors::default()
                .allowed_methods(vec!["GET", "POST"])
                .allowed_origin("https://doshtrade.com")
                .allowed_origin("http://doshtrade.com")
                .allowed_origin("wss://doshtrade.com")
                .allowed_origin("ws://doshtrade.com")
                .allow_any_header()
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
        .with_context(|| format!("actix-web failed"))
}
