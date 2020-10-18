#[derive(Debug, sqlx::FromRow)]
pub struct Stock {
    pub id: i32,
    pub ticker: String,
    pub name: String,
}

impl Stock {
    pub async fn find(conn: &crate::db::DbPoolConn, ticker_symbol: String) -> sqlx::Result<Stock> {
        sqlx::query_as::<_, Stock>("SELECT * FROM stocks WHERE id = ?")
            .bind(ticker_symbol)
            .fetch_one(conn)
            .await
    }
}
