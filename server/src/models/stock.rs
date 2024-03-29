#[derive(Debug, sqlx::FromRow)]
pub struct Stock {
    pub id: i32,
    pub ticker: String,
    pub name: String,
}

impl Stock {
    #[allow(dead_code)]
    pub async fn find(conn: &crate::db::DbPool, ticker_symbol: String) -> sqlx::Result<Stock> {
        sqlx::query_as::<_, Stock>("SELECT * FROM stocks WHERE ticker = $1")
            .bind(ticker_symbol)
            .fetch_one(conn)
            .await
    }

    pub async fn get_unique_tickers(conn: &crate::db::DbPool) -> sqlx::Result<Vec<String>> {
        sqlx::query_scalar("SELECT ticker FROM stocks")
            .fetch_all(conn)
            .await
    }

    pub async fn insert_ticker(conn: &crate::db::DbPool, ticker: &String) -> sqlx::Result<sqlx::postgres::PgDone> {
        sqlx::query("INSERT INTO stocks VALUES (DEFAULT, $1, $2)")
            .bind(ticker)
            .bind("FIXME DESCRIPTION")
            .execute(conn)
            .await
    }

    pub async fn tickers_to_stocks(conn: &crate::db::DbPool, tickers: &Vec<String>) -> Vec<Stock> {
        let queries = tickers.iter().map(|ticker|
            sqlx::query_as::<_, Stock>("SELECT * FROM stocks WHERE ticker = $1")
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
