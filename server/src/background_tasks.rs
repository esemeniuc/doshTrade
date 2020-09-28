use std::time::{Duration, SystemTime};

use actix::prelude::*;
use actix_cors::Cors;
use actix_web::client::Client;
use actix_web::{guard, web, App, HttpServer, Result};
use clap::{App as ClapApp, Arg};
use log::{error, info, trace, warn};
use serde::{Deserialize, Serialize};

use crate::asyncgql::{MutationRoot, QueryRoot, SubscriptionRoot};
use crate::models::IntradayPrice;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct IEXPrice {
    latest_price: f64,
    latest_volume: i64,
    latest_update: i64,
}

pub async fn background_send_push_notifications(conn: &crate::db::DbPoolConn) -> Result<(), ()> {
    let client = web_push::WebPushClient::new();

    //TODO: get all notifications from db
    //send demo message
    let subscription_info: web_push::SubscriptionInfo = unimplemented!();
    let message = crate::push_notification::generate_push_message(subscription_info)
        .expect("failed to generate push message");

    let response = client.send(message).await;
    response
        .map_err(|e| println!("got error in sendit(), {} ", e))
        .map(|result| println!("Got response: {:?}", result));
    Ok(())
}

pub async fn background_fetch_tickers(
    conn: &crate::db::DbPoolConn,
    tickers: Vec<String>,
) -> Result<(), actix_web::Error> {
    info!("Getting updates from IEX for {:#?}", tickers);
    trace!(
        "Started at {}",
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );

    let client = Client::default();
    // Create request builder and send request
    for ticker in tickers {
        info!("Fetching ticker: {}", ticker);
        let url = format!("https://sandbox.iexapis.com/stable/stock/{}/quote?filter=latestPrice,latestVolume,latestUpdate&token=Tsk_2311e67e08f1404498c7a7fb91685839", ticker);
        info!("Using url: {}", url);
        let mut response = client.get(url).send().await?;
        let body = response.body().await?;
        let price: IEXPrice = serde_json::from_slice(body.as_ref())?;
        info!("Downloaded: {:#?} ", price);

        let secs = price.latest_update / 1000; //time comes in as milliseconds, convert to sec
        let remaining_nanos = (price.latest_update % 1000) * 1_000_000;

        let query_result = IntradayPrice::insert(
            conn,
            &ticker,
            price.latest_price,
            price.latest_volume,
            chrono::NaiveDateTime::from_timestamp(secs, remaining_nanos as u32),
        );
        match query_result {
            Ok(_) => info!("Inserted intraday update for ticker: {}", ticker),
            Err(e) => warn!(
                "Failed to fetch intraday update for ticker: {} with error: {}",
                ticker, e
            ),
        }
    }
    trace!(
        "Ended at {}",
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );
    Ok(())
}

pub(crate) struct MyActor {
    pub(crate) pool: crate::db::DbPool,
}

impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        //background fetch
        ctx.run_interval(Duration::from_secs(10), move |this, ctx| {
            let conn = match this.pool.get() {
                Ok(v) => v,
                Err(e) => {
                    warn!("Failed to get pool conn connection {:?}", e);
                    return;
                }
            };

            ctx.spawn(actix::fut::wrap_future(async move {
                //spawns a separate task since we don't want to block based on prev request
                //TODO: find out which tickers are needed to fetch
                let tickers = vec!["AAPL".to_string(), "NFLX".to_string(), "GOOG".to_string()];
                match background_fetch_tickers(&conn, tickers).await {
                    Ok(_) => info!("Fetched all tickers"),
                    Err(e) => warn!("Failed to get data from iex, {}", e),
                }
            }));
        });

        //background send push notifications
        ctx.run_interval(Duration::from_secs(10), move |this, ctx| {
            let conn = match this.pool.get() {
                Ok(v) => v,
                Err(e) => {
                    warn!("Failed to get pool conn connection {:?}", e);
                    return;
                }
            };

            ctx.spawn(actix::fut::wrap_future(async move {
                //spawns a separate task since we don't want to block based on prev request
                match background_send_push_notifications(&conn).await {
                    Ok(_) => info!("Sent all push notifications"),
                    Err(e) => warn!("Failed to send notifications {:?}", e),
                }
            }));
        });
    }
}
