use std::time::{Duration, SystemTime};

use actix::prelude::*;
use actix_web::Result;
use log::{error, info, trace, warn};
use serde::{Deserialize, Serialize};
use web_push::SubscriptionKeys;

use crate::models::IntradayPrice;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct IEXPrice {
    latest_price: f64,
    //volume can be null before trading starts
    latest_volume: Option<i64>,
    latest_update: i64,
}

pub async fn background_send_push_notifications(
    conn: &crate::db::DbPool,
) -> Result<(), sqlx::Error> {
    let client = web_push::WebPushClient::new();
    #[derive(sqlx::FromRow)]
    struct ClientSubscription {
        stock_id: i32,
        endpoint: String,
        p256dh: String,
        auth: String,
    }
    let client_subs = sqlx::query_as::<_, ClientSubscription>(
        "SELECT stock_id, endpoint, p256dh, auth FROM client_subscriptions
        JOIN clients ON clients.id = client_subscriptions.client_id", //need the client info for notification
    )
        .fetch_all(conn)
        .await?;

    let first_client = client_subs.first().unwrap(); //FIXME
    let rsi_val = crate::models::IntradayPrice::get_rsi_by_stock_id(conn, first_client.stock_id, 14).await.unwrap();
    if rsi_val <= 0.15 || rsi_val >= 0.85 {
        let sub = web_push::SubscriptionInfo {
            endpoint: first_client.endpoint.to_owned(),
            keys: SubscriptionKeys {
                p256dh: first_client.p256dh.to_owned(),
                auth: first_client.auth.to_owned(),
            },
        };
        let msg = crate::push_notification::generate_push_message(sub).unwrap();
        client.send(msg).await;
    }

    Ok(())

    // let messages_to_send = client_subs
    //     .into_iter()
    //     .map(|sub| (
    //         sub.stock_id,
    //         web_push::SubscriptionInfo {
    //             endpoint: sub.endpoint,
    //             keys: SubscriptionKeys {
    //                 p256dh: sub.p256dh,
    //                 auth: sub.auth,
    //             },
    //         },
    //     ))
    //     .filter_map(|sub|
    //                     crate::push_notification::generate_push_message(sub.1).ok() //TODO: use ticker info in messages!
    //     )
    //     .map(|msg| client.send(msg));
    //
    // //send it!
    // let send_results = futures::future::join_all(messages_to_send).await;
    // let (_, errs): (Vec<_>, Vec<_>) = itertools::Itertools::partition_map(send_results.into_iter(), |r| match r {
    //     Ok(v) => itertools::Either::Left(v),
    //     Err(v) => itertools::Either::Right(v),
    // });
    // errs.iter().for_each(|x| log::error!("Failed to send push message: {}", x));
    // Ok(())
}

pub async fn background_fetch_tickers(
    conn: &crate::db::DbPool,
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
            price.latest_volume.unwrap_or_default(),
            chrono::NaiveDateTime::from_timestamp(secs, remaining_nanos as u32),
        )
            .await;
        match query_result {
            Ok(_) => info!("Inserted intraday update for ticker: {}", ticker),
            Err(e) => error!(
                "Failed to fetch intraday update for ticker: {} with error: {:?}",
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
            let conn = this.pool.to_owned();
            ctx.spawn(actix::fut::wrap_future(async move {
                //spawns a separate task since we don't want to block based on prev request
                //TODO: find out which tickers are needed to fetch
                let tickers = vec![
                    "AAPL".to_string(),
                    "GE".to_string(),
                    "GOOG".to_string(),
                    "NFLX".to_string(),
                ];


                //example
                // userA, [A,B,C] -> 3 rows in db
                // userB, [B,C] -> 2 rows in db

                //select distinct(stockticker) from subscriptions
                //returns [A,B,C]

                //call iex with this
                //insert price data into intraday_prices table

                //periodically scan subscriptions table
                //calculate if a watched ticker should notify based on each row
                //
                //do calculation using intraday_prices table
                //eg userA with stock ticker B ($$$TICKER)
                //db: select the intraday updates needed for calculation
                // SELECT price, volume FROM intraday_updates
                // JOIN stocks ON stocks.id = intraday_prices.id
                // WHERE stocks.ticker = $$$TICKER
                // ORDER by timestamp DESC
                // LIMIT 5 (whatever is actually necessary for calc)

                match background_fetch_tickers(&conn, tickers).await {
                    Ok(_) => info!("Fetched all tickers"),
                    Err(e) => warn!("Failed to get data from IEX, {:?}", e),
                }
            }));
        });

        //background send push notifications
        ctx.run_interval(Duration::from_secs(10), move |this, ctx| {
            let conn = this.pool.to_owned();
            ctx.spawn(actix::fut::wrap_future(async move {
                //spawns a separate task since we don't want to block based on prev request
                log::trace!("Sending background push notifications!");
                match background_send_push_notifications(&conn).await {
                    Ok(_) => log::info!("Completed sending background push notifications"),
                    Err(e) => log::error!("Error sending background push: {}", e)
                }
            }));
        });
    }
}
