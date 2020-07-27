use crate::models::schema::intraday_prices;
use crate::models::schema::intraday_prices::dsl::*;
use diesel::prelude::*;

#[derive(Identifiable, Queryable, Debug)]
pub struct IntradayPrice {
    pub id: i32,
    pub stock_id: i32,
    pub price: f64,
    pub volume: i64,
    pub timestamp: chrono::NaiveDateTime,
}

impl IntradayPrice {
    pub fn get_latest(
        conn: &crate::db::DbPoolConn,
        other_stock_id: i32,
    ) -> QueryResult<IntradayPrice> {
        intraday_prices
            .filter(stock_id.eq(other_stock_id))
            .order(timestamp.desc())
            .first::<IntradayPrice>(conn)
    }

    pub fn insert(
        conn: &crate::db::DbPoolConn,
        other_stock_ticker: &String,
        other_price: f64,
        other_volume: i64,
        other_timestamp: chrono::NaiveDateTime,
    ) -> QueryResult<usize> {
        crate::models::Stock::find(conn, other_stock_ticker).and_then(|stock| {
            diesel::insert_into(intraday_prices::table)
                .values((
                    stock_id.eq(stock.id),
                    price.eq(other_price),
                    volume.eq(other_volume),
                    timestamp.eq(other_timestamp),
                ))
                .execute(conn)
        })
    }
}
