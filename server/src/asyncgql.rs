use std::time::Duration;

use async_graphql::{Context, Schema, ID};
use futures::{Stream, StreamExt};
use itertools::Itertools;

use crate::models::{Client, ClientSubscription, IntradayPrice, Stock as DbStock};

pub type BooksSchema = Schema<QueryRoot, MutationRoot, Subscription>;

#[derive(Clone)]
pub struct Book {
    id: ID,
    name: String,
    author: String,
}

#[async_graphql::Object]
impl Book {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn author(&self) -> &str {
        &self.author
    }
}

pub struct QueryRoot;

#[derive(async_graphql::SimpleObject, Clone)]
struct OptionQuote {
    option_type: crate::models::option::OptionType,
    strike: Option<f64>,
    expiration: String,
    bid: Option<f64>,
    ask: Option<f64>,
    last: Option<f64>,
    delta: f64,
    gamma: f64,
    theta: f64,
    vega: f64,
    rho: f64,
    volatility: f64,
    time_value: f64,
}

#[async_graphql::Object]
impl QueryRoot {
    ///sends demo notification to client browser to verify notifications work as intended
    async fn send_demo_notification(
        &self,
        push_subscription: crate::push_notification::PushSubscription,
    ) -> bool {
        log::trace!("Sending push subscription!: {:?}", push_subscription);
        let subscription_info = web_push::SubscriptionInfo::from(push_subscription.clone());

        let message = match crate::push_notification::generate_push_message(subscription_info, "this is a demo message") {
            Ok(v) => v,
            Err(e) => {
                log::error!("failed to generate push message: {}", e);
                return false;
            }
        };

        match crate::push_notification::send_demo(message).await {
            Ok(_) => true,
            Err(e) => {
                log::error!("failed to send push message: {}", e);
                return false;
            }
        }
    }

    ///sends option chain for selected ticker
    async fn get_option_chain(
        &self,
        ticker: String,
    ) -> Vec<OptionQuote> {
        vec![OptionQuote {
            option_type: crate::models::option::OptionType::Call,
            strike: None,
            expiration: "".to_string(),
            bid: None,
            ask: None,
            last: None,
            delta: 0.0,
            gamma: 0.0,
            theta: 0.0,
            vega: 0.0,
            rho: 0.0,
            volatility: 0.0,
            time_value: 0.0,
        }]
    }
}

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    ///Returns a list of successfully added tickers.
    ///Invalid (not found) tickers will not be returned.
    //TODO handle unsubscribe action
    async fn notification_request(
        &self,
        ctx: &Context<'_>,
        ticker_symbols: Vec<String>,
        push_subscription: crate::push_notification::PushSubscription,
    ) -> Vec<String> {
        let pool = ctx.data_unchecked::<crate::db::DbPool>();

        //add user to client table
        let client_id = match Client::upsert(pool, &push_subscription).await {
            Ok(id) => id,
            Err(e) => {
                log::warn!("notification_request() failed with error: {}", e);
                return vec![];
            }
        };

        //TODO: insert subscription where not in table rather than delete (faster maybe!)
        match ClientSubscription::delete_all(pool, client_id).await {
            Ok(_) => (),
            Err(e) => {
                log::warn!("notification_request() failed with error: {}", e);
                return vec![];
            }
        };

        let valid_stocks = DbStock::tickers_to_stocks(pool, &ticker_symbols).await;
        let inserts = valid_stocks.iter()
            .map(|stock| ClientSubscription::insert(pool, client_id, stock.id));
        let query_results = futures::future::join_all(inserts).await;

        let (oks, errs): (Vec<_>, Vec<_>) = query_results
            .into_iter()
            .zip(valid_stocks)
            .partition_map(|r| match r.0 {
                Ok(_) => itertools::Either::Left(r.1.ticker),
                Err(v) => itertools::Either::Right(v),
            });
        errs.iter().for_each(|x| log::error!("Failed to insert client subscription for ticker: {}", x));
        oks
    }
}

#[derive(async_graphql::SimpleObject, Clone)]
///Represents a stock's status
struct Stock {
    ticker: String,
    price: String,
    rsi: f64,
    ///% Change from the start of day
    percent_change: f64,
    timestamp: String,
}

pub struct Subscription;

#[async_graphql::Subscription]
impl Subscription {
    async fn yolo_hand_curated_stocks(
        &self,
        ctx: &Context<'_>,
        ticker_symbols: Vec<String>,
    ) -> impl Stream<Item=Vec<Stock>> {
        use std::sync::Arc;
        let conn_owned = Arc::new(ctx.data_unchecked::<sqlx::PgPool>().to_owned());
        let tickers_owned = Arc::new(ticker_symbols);

        actix_web::rt::time::interval(Duration::from_secs(5))
            .map(move |_| (Arc::clone(&conn_owned), Arc::clone(&tickers_owned)))
            .then(|vars| {
                async move {
                    let rsi_interval = 14;
                    let prices = IntradayPrice::get_latest_by_tickers(&vars.0, &vars.1).await;
                    let rsi_vals = IntradayPrice::get_rsi_by_tickers(&vars.0, &vars.1, rsi_interval).await;
                    let open_prices = IntradayPrice::get_open_prices_by_stock_ids(&vars.0,
                                                                                  &prices.iter().map(|x| (x.stock_id, x.timestamp)).collect::<Vec<_>>()).await;
                    prices.iter().zip(rsi_vals).zip(open_prices)
                        .map(|x| {
                            let ((intraday_price, rsi), open_price) = x;
                            Stock {
                                ticker: intraday_price.ticker.clone(),
                                price: format!("{:.2}", intraday_price.price),
                                rsi,
                                percent_change: 100.0 * ((intraday_price.price / open_price) - 1.0),
                                timestamp: intraday_price.timestamp.to_string(),
                            }
                        })
                        .collect::<Vec<_>>()
                }
            })

        // actix_web::rt::time::interval(Duration::from_secs(5))
        //     .then(|_| async move {
        //         vec![Stock {
        //             ticker: "".to_string(),
        //             price: "".to_string(),
        //             rsi: 0.0,
        //             percent_change: 0.0,
        //             timestamp: "".to_string(),
        //         }]
        //     })


        // actix_web::rt::time::interval(Duration::from_secs(5)).then(move |_| {
        //     let b = futures::stream::iter(ticker_symbols.to_owned().into_iter());
        //     let c = b.then(|ticker| async {
        //         (
        //             IntradayPrice::get_latest_by_ticker(&conn, &ticker).await,
        //             ticker,
        //         )
        //     });
        //     let d = c.filter_map(|ticker_and_intraday_price| async {
        //         match ticker_and_intraday_price.0 {
        //             Ok(intraday_price) => Some(Stock {
        //                 ticker: ticker_and_intraday_price.1.to_owned(),
        //                 price: intraday_price.price.to_string(),
        //                 rsi: 0.1,            //TODO: calculate this
        //                 percent_change: 0.2, //TODO: calculate this
        //                 timestamp: intraday_price.timestamp.to_string(),
        //             }),
        //             Err(e) => None,
        //         }
        //     });
        //     let e = d.collect::<Vec<_>>();
        //     return e;
        // })
    }
}
