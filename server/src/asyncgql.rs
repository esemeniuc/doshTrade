use async_graphql::*;
//for field macro
use crate::models::Client;
use async_graphql::{Context, FieldResult, Schema, SimpleBroker, ID};
use futures::lock::Mutex;
use futures::{Stream, StreamExt};
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
impl QueryRoot {}

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    async fn notification_request(
        &self,
        ctx: &Context<'_>,
        ticker_symbols: Vec<String>,
        push_subscription: crate::push_notification::PushSubscription,
    ) -> bool {
        let pool = ctx.data::<crate::db::DbPool>();

        //store ticker and subscriptions
        let subscription_info = web_push::SubscriptionInfo::from(push_subscription.clone());
        //add user to client table
        //cannot have duplicates due to unique constraint
        Client::insert(&pool.get().unwrap(), push_subscription);

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

#[async_graphql::SimpleObject(desc = "Represents a stock status")]
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
        ticker_symbols: Vec<String>,
    ) -> impl Stream<Item = Vec<Stock>> {
        let prices = ticker_symbols
            .into_iter()
            .map(|ticker| Stock {
                ticker,
                price: "666.66".to_string(),
                rsi: 0.1,
                percent_change: 0.2,
                timestamp: "12345".to_string(),
            })
            .collect();
        futures::stream::once(async { prices })
    }
}
