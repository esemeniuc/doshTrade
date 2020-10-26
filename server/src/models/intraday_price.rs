use itertools::Itertools;
use sqlx::sqlite::SqliteDone;

#[derive(sqlx::FromRow, Debug)]
pub struct IntradayPrice {
    pub id: i32,
    pub stock_id: i32,
    pub price: f64,
    pub volume: i64,
    pub timestamp: chrono::NaiveDateTime,
    pub ticker: String,
}

impl IntradayPrice {
    //returns successfully found tickers
    pub async fn get_latest_by_tickers(
        conn: &crate::db::DbPoolConn,
        tickers: &Vec<String>,
    ) -> Vec<IntradayPrice> {
        let price_queries = tickers
            .iter()
            .map(|ticker| IntradayPrice::get_latest_by_ticker(conn, ticker));
        let query_results = futures::future::join_all(price_queries).await;
        let (oks, errs): (Vec<_>, Vec<_>) = query_results.into_iter().partition_map(|r| match r {
            Ok(v) => itertools::Either::Left(v),
            Err(v) => itertools::Either::Right(v),
        });
        errs.iter()
            .for_each(|x| log::error!("Failed to find ticker: {}", x));
        oks
    }

    pub async fn get_latest_by_ticker(
        conn: &crate::db::DbPoolConn,
        ticker: &String, //TODO check if ref is ok
    ) -> sqlx::Result<IntradayPrice> {
        sqlx::query_as::<_, IntradayPrice>(
            "SELECT intraday_prices.id,
             intraday_prices.stock_id,
             ROUND(intraday_prices.price,2) as price,
             intraday_prices.volume,
             intraday_prices.timestamp,
             stocks.ticker
         FROM intraday_prices AS intraday_prices
         JOIN stocks ON stocks.id = intraday_prices.stock_id AND stocks.ticker = ?
         ORDER BY intraday_prices.timestamp DESC
         LIMIT 1",
        )
        .bind(ticker)
        .fetch_one(conn)
        .await
    }

    pub async fn insert(
        conn: &crate::db::DbPoolConn,
        other_stock_ticker: &String,
        other_price: f64,
        other_volume: i64,
        other_timestamp: chrono::NaiveDateTime,
    ) -> sqlx::Result<SqliteDone> {
        sqlx::query("INSERT INTO intraday_prices VALUES (null, (SELECT id from stocks where ticker = ?), ?, ?, ?)")
            .bind(other_stock_ticker)
            .bind(other_price)
            .bind(other_volume)
            .bind(other_timestamp)
            .execute(conn)
            .await
    }
}
