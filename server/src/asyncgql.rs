use async_graphql::*;
//for field macro
use crate::models::schema::intraday_prices::dsl::intraday_prices;
use crate::models::{Client, ClientSubscription, IntradayPrice, Stock as DbStock};
use async_graphql::{Context, FieldResult, Schema, SimpleBroker, ID};
use diesel::QueryResult;
use futures::lock::Mutex;
use futures::{FutureExt, Stream, StreamExt};
use log::{error, info, trace, warn};
use slab::Slab;
use std::sync::Arc;
use std::time::Duration;

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

pub type Storage = Arc<Mutex<Slab<Book>>>;

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn get_debug(
        &self,
        push_subscription: crate::push_notification::PushSubscription,
    ) -> bool {
        println!("push_subscription: {:?}", push_subscription);
        // TODO:
        let subscription_info = web_push::SubscriptionInfo::from(push_subscription.clone());
        let message = crate::push_notification::generate_push_message(subscription_info)
            .expect("failed to generate push message");

        crate::push_notification::send_it(message).await;
        true
    }
}

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    async fn notification_request(
        &self,
        ctx: &Context<'_>,
        ticker_symbols: Vec<String>,
        push_subscription: crate::push_notification::PushSubscription,
    ) -> bool {
        let pool = match ctx.data::<crate::db::DbPool>() {
            Ok(val) => val,
            Err(e) => {
                error!("Error getting db pool from context: {}", e.0);
                return false;
            }
        };
        let conn = pool.get().unwrap();

        //store ticker and subscriptions
        let subscription_info = web_push::SubscriptionInfo::from(push_subscription.clone());
        //add user to client table
        //cannot have duplicates due to unique constraint
        Client::insert(&conn, &push_subscription);
        let client_id = 1; //TODO get client
        for ticker in ticker_symbols.iter() {
            let stock_id = 1; //TODO get stock
            if let Err(e) = ClientSubscription::insert(&conn, client_id, stock_id) {
                error!("Error inserting client_subscription row: {}", e);
                return false;
            }
        }
        //delete all previous tickers for the user
        //create row for each ticker

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

        //send demo message

        //send demo message
        let message = crate::push_notification::generate_push_message(subscription_info)
            .expect("failed to generate push message");

        crate::push_notification::send_it(message).await;
        true
    }
}

#[async_graphql::SimpleObject(desc = "Represents a stock's status")]
#[derive(Clone)]
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
