use std::time::{Duration, SystemTime};

use actix::prelude::*;
use actix_web::Result;
use log::{error, info, trace, warn};
use web_push::SubscriptionKeys;

use crate::models::IntradayPrice;

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockQuote {
    pub asset_type: String,
    pub asset_main_type: String,
    pub cusip: String,
    pub symbol: String,
    pub description: String,
    pub bid_price: f64,
    pub bid_size: i64,
    pub bid_id: String,
    pub ask_price: f64,
    pub ask_size: i64,
    pub ask_id: String,
    pub last_price: f64,
    pub last_size: i64,
    pub last_id: String,
    pub open_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub bid_tick: String,
    pub close_price: f64,
    pub net_change: f64,
    pub total_volume: i64,
    pub quote_time_in_long: i64,
    pub trade_time_in_long: i64,
    pub mark: f64,
    pub exchange: String,
    pub exchange_name: String,
    pub marginable: bool,
    pub shortable: bool,
    pub volatility: f64,
    pub digits: i64,
    #[serde(rename = "52WkHigh")]
    pub n52_wk_high: f64,
    #[serde(rename = "52WkLow")]
    pub n52_wk_low: f64,
    #[serde(rename = "nAV")]
    pub n_av: f64,
    pub pe_ratio: f64,
    pub div_amount: f64,
    pub div_yield: f64,
    pub div_date: String,
    pub security_status: String,
    pub regular_market_last_price: f64,
    pub regular_market_last_size: i64,
    pub regular_market_net_change: f64,
    pub regular_market_trade_time_in_long: i64,
    pub net_percent_change_in_double: f64,
    pub mark_change_in_double: f64,
    pub mark_percent_change_in_double: f64,
    pub regular_market_percent_change_in_double: f64,
    pub delayed: bool,
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
        JOIN clients ON clients.id = client_subscriptions.client_id") //need the client info for notification
        .fetch_all(conn)
        .await?;

    let client_subs_fut = client_subs
        .into_iter()
        .map(|sub| async {
            (crate::models::IntradayPrice::get_rsi_by_stock_id(conn, sub.stock_id, 14).await, sub)
        });

    let messages_to_send = futures::future::join_all(client_subs_fut)
        .await
        .into_iter()
        .filter_map(|x| match x.0 {
            Ok(val) => Some((val, x.1)),
            Err(_) => None
        })
        .filter_map(|x| {
            let (rsi_val, sub) = x;
            let notification_msg = if rsi_val <= 0.15 {
                format!("id {} is oversold", sub.stock_id)
            } else if rsi_val >= 0.51 {
                format!("id {} is overbought", sub.stock_id)
            } else {
                return None;
            };

            let sub_info = web_push::SubscriptionInfo {
                endpoint: sub.endpoint,
                keys: SubscriptionKeys {
                    p256dh: sub.p256dh,
                    auth: sub.auth,
                },
            };
            crate::push_notification::generate_push_message(sub_info, &notification_msg).ok()
        })
        .map(|msg| client.send(msg));

    //send it!
    let send_results = futures::future::join_all(messages_to_send).await;
    let (_, errs): (Vec<_>, Vec<_>) = itertools::Itertools::partition_map(
        send_results.into_iter(),
        |r| match r {
            Ok(v) => itertools::Either::Left(v),
            Err(v) => itertools::Either::Right(v),
        });
    errs.iter().for_each(|x| log::error!("Failed to send push message: {}", x));
    Ok(())
}

pub async fn background_fetch_tickers(
    conn: &crate::db::DbPool,
    tickers: Vec<String>,
) -> Result<(), actix_web::Error> {
    info!("Getting updates from TD for {:#?}", tickers);
    trace!(
        "Started at {}",
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );

    let client = actix_web::client::Client::default();
    // Create request builder and send request
    info!("Fetching tickers: {:?}", tickers);
    let tickers_str = tickers.join(",");
    let url = format!("https://api.tdameritrade.com/v1/marketdata/quotes?apikey=YPUACAREWAHFTZDFPJJ0FKWN8B7NVVHF&symbol={}", tickers_str);
    info!("Using url: {}", url);
    let mut response = client.get(url).send().await?;
    let body = response.body().await?;
    let quotes: std::collections::HashMap<String, StockQuote> = serde_json::from_slice(body.as_ref())?;
    let quotes2 = quotes.into_iter().map(|(_k, v)| v).collect::<Vec<StockQuote>>();
    let query_result = IntradayPrice::insert_many(conn, quotes2).await;
    // match query_result {
    //     Ok(_) => info!("Inserted intraday update for tickers: {}", tickers),
    //     Err(e) => error!("Failed to fetch intraday update with error: {:?}", e),
    // }

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
                let tickers = vec!["AAPL".to_string(),
                                   "FB".to_string(),
                                   "GLD".to_string(),
                                   "GOOG".to_string(),
                                   "LIT".to_string(),
                                   "NFLX".to_string(),
                                   "SLV".to_string(),
                                   "SQ".to_string(),
                                   "TSLA".to_string(),
                                   "TSM".to_string(),
                                   "UVXY".to_string(),
                                   "ZM".to_string()];
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
                    Err(e) => warn!("Failed to get data from TD, {:?}", e),
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
