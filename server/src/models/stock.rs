#[derive(Debug, sqlx::FromRow)]
pub struct Stock {
    pub id: i32,
    pub ticker: String,
    pub name: String,
}

impl Stock {
    pub async fn find(conn: &crate::db::DbPoolConn, ticker_symbol: String) -> sqlx::Result<Stock> {
        sqlx::query_as::<_, Stock>("SELECT * FROM stocks WHERE ticker = ?")
            .bind(ticker_symbol)
            .fetch_one(conn)
            .await
    }

    pub async fn tickers_to_stocks(conn: &crate::db::DbPoolConn, tickers: &Vec<String>) -> Vec<Stock> {
        let queries = tickers.iter().map(|ticker|
            sqlx::query_as::<_, Stock>("SELECT * FROM stocks WHERE ticker = ?")
                .bind(ticker)
                .fetch_one(conn)
        );

        futures::future::join_all(queries)
            .await
            .into_iter()
            .filter_map(Result::ok)
            .collect::<Vec<_>>()
    }
}
