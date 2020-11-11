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

#[async_graphql::Object]
impl QueryRoot {
    ///sends demo notification to client browser to verify notifications work as intended
    async fn send_demo_notification(
        &self,
        push_subscription: crate::push_notification::PushSubscription,
    ) -> bool {
        log::trace!("Sending push subscription!: {:?}", push_subscription);
        let subscription_info = web_push::SubscriptionInfo::from(push_subscription.clone());
        let message = match crate::push_notification::generate_push_message(subscription_info) {
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
        let conn_owned = Arc::new(ctx.data_unchecked::<sqlx::SqlitePool>().to_owned());
        let tickers_owned = Arc::new(ticker_symbols);

        actix_web::rt::time::interval(Duration::from_secs(5))
            .map(move |_| (Arc::clone(&conn_owned), Arc::clone(&tickers_owned)))
            .then(|vars| {
                async move {
                    let prices = IntradayPrice::get_latest_by_tickers(&vars.0, &vars.1).await;
                    let rsis = IntradayPrice::get_rsi_by_tickers(&vars.0, &vars.1).await;
                    prices.iter().zip(rsis)
                        .map(|x| {
                            let (intraday_price, rsi) = x;
                            Stock {
                                ticker: intraday_price.ticker.clone(),
                                price: intraday_price.price.to_string(), //TODO: format this
                                rsi,
                                percent_change: 0.2, //TODO: calculate this
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
