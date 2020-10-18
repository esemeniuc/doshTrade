use sqlx::sqlite::SqliteDone;

#[derive(sqlx::FromRow, Debug)]
pub struct IntradayPrice {
    pub id: i32,
    pub stock_id: i32,
    pub price: f64,
    pub volume: i64,
    pub timestamp: chrono::NaiveDateTime,
}

impl IntradayPrice {
    pub async fn get_latest_by_id(
        conn: &crate::db::DbPoolConn,
        other_stock_id: i32,
    ) -> sqlx::Result<IntradayPrice> {
        sqlx::query_as::<_, IntradayPrice>(
            "SELECT * FROM intraday_prices
         WHERE stock_id = ?
         ORDER BY timestamp DESC
         LIMIT 1",
        )
        .bind(other_stock_id)
        .fetch_one(conn)
        .await
    }

    pub async fn get_latest_by_ticker(
        conn: &crate::db::DbPoolConn,
        ticker: &String, //TODO check if ref is ok
    ) -> sqlx::Result<IntradayPrice> {
        sqlx::query_as::<_, IntradayPrice>(
            "SELECT * FROM intraday_prices
         JOIN stocks ON stocks.ticker = ?
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
