use crate::models::schema::intraday_prices;
use crate::models::schema::intraday_prices::dsl::*;
use diesel::prelude::*;

#[derive(Identifiable, Queryable, Debug)]
pub struct IntradayPrice {
    pub id: i32,
    pub stock_id: i32,
    pub price: f64,
    pub volume: i32,
    pub timestamp: chrono::NaiveDateTime,
}

impl IntradayPrice {
    pub fn get_latest(
        conn: &crate::db::DbPoolConn,
        query_stock_id: i32,
    ) -> QueryResult<IntradayPrice> {
        intraday_prices.filter(stock_id.eq(query_stock_id))
            .order(timestamp.desc())
            .first::<IntradayPrice>(conn)
    }
}
