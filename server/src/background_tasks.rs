use std::time::{Duration, SystemTime};

use actix::prelude::*;
use actix_web::Result;
use clap::{App as ClapApp, Arg};
use log::{error, info, trace, warn};
use serde::{Deserialize, Serialize};

use crate::models::schema::client_subscriptions::dsl::*;
use crate::models::schema::clients::dsl::*;
use crate::models::{Client, IntradayPrice};
use diesel::{QueryDsl, RunQueryDsl, Table};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct IEXPrice {
    latest_price: f64,
    latest_volume: i64,
    latest_update: i64,
}

pub async fn background_send_push_notifications(conn: &crate::db::DbPoolConn) {
    let client = web_push::WebPushClient::new();
    let db_clients: Vec<Client> = match client_subscriptions
        .inner_join(clients) //get the client
        //TODO: get actual stock info!
        .select(clients::all_columns()) //need the client cols for destination, need stock tickers
        .load(conn)
    {
        Ok(val) => val,
        Err(e) => {
            warn!("Failed to fetch client subs from db with error {}", e);
            return;
        }
    };

    let messages_to_send: Vec<_> = db_clients
        .into_iter()
        .map(|sub| web_push::SubscriptionInfo::from(sub))
        .filter_map(|sub| crate::push_notification::generate_push_message(sub).ok())
        .map(|msg| client.send(msg))
        .collect();
    //send it!
    let send_results = futures::future::join_all(messages_to_send).await;
    let send_errors: Vec<_> = send_results
        .iter()
        .filter(|elem| Result::is_err(elem))
        .collect();
    match send_errors.len() {
        0 => info!(
            "Successfully sent all {} push notifications",
            send_results.len()
        ),
        _ => warn!(
            "background_send_push_notifications() failed to send {} push notifications",
            send_errors.len()
        ),
    }
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

    let client = actix_web::client::Client::default();
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
                    Err(e) => warn!("Failed to get data from IEX, {}", e),
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
                background_send_push_notifications(&conn).await
            }));
        });
    }
}
