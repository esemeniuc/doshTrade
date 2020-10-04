use std::time::Duration;

use async_graphql::*;
use async_graphql::{Context, FieldResult, Schema, ID};
use diesel::QueryResult;
use futures::{Stream, StreamExt};
use log::{error, info, trace, warn};

use crate::models::{Client, ClientSubscription, IntradayPrice, Stock as DbStock};

pub type BooksSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

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
        trace!("Sending push subscription!: {:?}", push_subscription);
        let subscription_info = web_push::SubscriptionInfo::from(push_subscription.clone());
        let message = crate::push_notification::generate_push_message(subscription_info)
            .expect("failed to generate push message");

        crate::push_notification::send_demo(message).await;
        true
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
        let pool = match ctx.data::<crate::db::DbPool>() {
            Ok(val) => val,
            Err(e) => {
                error!("Error getting db pool from context: {}", e.0);
                return vec![];
            }
        };

        let successful_tickers = || -> QueryResult<Vec<String>> {
            let conn = pool.get().unwrap();
            //add user to client table
            let client = Client::upsert(&conn, &push_subscription)?;
            ClientSubscription::delete_all(&conn, client.id)?;
            let output: Vec<_> = ticker_symbols
                .iter()
                .filter_map(|ticker| crate::models::Stock::find(&conn, ticker).ok())
                .filter_map(
                    //store ticker and subscription
                    |stock| match ClientSubscription::insert(&conn, client.id, stock.id) {
                        Ok(_) => Some(stock.ticker),
                        Err(_) => None,
                    },
                )
                .collect();
            Ok(output)
        };

        return match successful_tickers() {
            Ok(val) => val,
            Err(e) => {
                warn!("notification_request() failed with code{}", e);
                vec![]
            }
        };

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
    }
}

#[derive(async_graphql::SimpleObject, Clone)]
#[graphql(desc = "Represents a stock's status")]
struct Stock {
    ticker: String,
    price: String,
    #[field(desc = "TODO: figure out RSI")]
    rsi: f64,
    #[field(desc = "% Change from the start of day")]
    percent_change: f64,
    timestamp: String,
}

pub struct SubscriptionRoot;

#[async_graphql::Subscription]
impl SubscriptionRoot {
    async fn yolo_hand_curated_stocks(
        &self,
        ctx: &Context<'_>,
        ticker_symbols: Vec<String>,
    ) -> FieldResult<impl Stream<Item = Vec<Stock>>> {
        let conn = ctx
            .data::<crate::db::DbPool>()
            .and_then(|pool| pool.get().map_err(|e| FieldError::from(e)));

        fn get_price(conn: &crate::db::DbPoolConn, ticker: &String) -> QueryResult<Stock> {
            DbStock::find(&conn, &ticker)
                .and_then(|stock| IntradayPrice::get_latest(&conn, stock.id))
                .map(|intraday_price| Stock {
                    ticker: ticker.to_owned(),
                    price: intraday_price.price.to_string(),
                    rsi: 0.1,            //TODO: calculate this
                    percent_change: 0.2, //TODO: calculate this
                    timestamp: intraday_price.timestamp.to_string(),
                })
        }
        conn.map(|conn| {
            tokio::time::interval(Duration::from_secs(5)).map(move |_| {
                ticker_symbols
                    .iter()
                    .filter_map(|ticker| get_price(&conn, ticker).ok())
                    .collect::<Vec<Stock>>()
            })
        })
    }
}
