use async_graphql::*;
//for field macro
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

#[async_graphql::InputObject]
struct PushSubscriptionKeys {
    p256dh: String,
    auth: String,
}

#[async_graphql::InputObject]
struct PushSubscription {
    endpoint: String,
    expiration_time: Option<String>,
    keys: PushSubscriptionKeys,
}

#[async_graphql::Object]
impl MutationRoot {
    async fn notification_request(
        &self,
        ticker_symbols: Vec<String>,
        push_subscription: PushSubscription,
    ) -> bool {
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
