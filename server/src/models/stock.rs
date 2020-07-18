use crate::models::schema::stocks;
use crate::models::schema::stocks::dsl::*;
use diesel::prelude::*;

#[derive(Identifiable, Queryable, Debug)]
pub struct Stock {
    pub id: i32,
    pub ticker: String,
    pub name: String,
}

impl Stock {
    pub fn find(
        conn: &crate::db::DbPoolConn,
        ticker_symbol: &String,
    ) -> QueryResult<Stock> {
        stocks.filter(ticker.eq(ticker_symbol))
            .first::<Stock>(conn)
    }
}
