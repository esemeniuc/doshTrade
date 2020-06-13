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
impl QueryRoot {
    async fn books(&self, ctx: &Context<'_>) -> Vec<Book> {
        let books = ctx.data::<Storage>().lock().await;
        books.iter().map(|(_, book)| book).cloned().collect()
    }
}

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    async fn create_book(&self, ctx: &Context<'_>, name: String, author: String) -> ID {
        let mut books = ctx.data::<Storage>().lock().await;
        let entry = books.vacant_entry();
        let id: ID = entry.key().into();
        let book = Book {
            id: id.clone(),
            name,
            author,
        };
        entry.insert(book);
        SimpleBroker::publish(BookChanged {
            mutation_type: MutationType::Created,
            id: id.clone(),
        });
        id
    }

    async fn delete_book(&self, ctx: &Context<'_>, id: ID) -> FieldResult<bool> {
        let mut books = ctx.data::<Storage>().lock().await;
        let id = id.parse::<usize>()?;
        if books.contains(id) {
            books.remove(id);
            SimpleBroker::publish(BookChanged {
                mutation_type: MutationType::Deleted,
                id: id.into(),
            });
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

#[async_graphql::Enum]
enum MutationType {
    Created,
    Deleted,
}

#[async_graphql::SimpleObject]
#[derive(Clone)]
struct BookChanged {
    mutation_type: MutationType,
    id: ID,
}

#[async_graphql::SimpleObject]
#[derive(Clone)]
struct Stock {
    ticker: String,
    price: String,
    timestamp: String,
}


pub struct SubscriptionRoot;

#[async_graphql::Subscription]
impl SubscriptionRoot {
    async fn stock_prices(&self, ticker_symbols: Vec<String>) -> impl Stream<Item=Vec<Stock>> {
        let prices = ticker_symbols.into_iter().map(|ticker|
            Stock {
                ticker,
                price: "666.66".to_string(),
                timestamp: "12345".to_string(),
            }).collect();
        futures::stream::once(async { prices })
    }

    async fn reversal_alerts(&self, ticker_symbols: Vec<String>) -> impl Stream<Item=Vec<Stock>> {
        let prices = ticker_symbols.into_iter().map(|ticker|
            Stock {
                ticker,
                price: "666.66".to_string(),
                timestamp: "12345".to_string(),
            }).collect();
        futures::stream::once(async { prices })
    }

    async fn oversold_stocks(&self) -> impl Stream<Item=Vec<Stock>> {
        let prices = vec!["GOOG", "AAPL"].into_iter().map(|ticker|
            Stock {
                ticker: ticker.to_string(),
                price: "666.66".to_string(),
                timestamp: "12345".to_string(),
            }).collect();
        futures::stream::once(async { prices })
    }

}
