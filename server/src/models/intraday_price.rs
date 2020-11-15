use itertools::Itertools;

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
        conn: &crate::db::DbPool,
        tickers: &Vec<String>,
    ) -> Vec<IntradayPrice> {
        let price_queries = tickers
            .iter()
            .map(|ticker| IntradayPrice::get_latest_by_ticker(conn, ticker));
        let query_results = futures::future::join_all(price_queries).await;
        let (oks, errs): (Vec<_>, Vec<_>) = query_results
            .into_iter()
            .partition_map(|r| match r {
                Ok(v) => itertools::Either::Left(v),
                Err(v) => itertools::Either::Right(v),
            });
        errs.iter().for_each(|x| log::error!("get_latest_by_tickers(): Failed to find ticker: {}", x));
        oks
    }

    pub async fn get_latest_by_ticker(
        conn: &crate::db::DbPool,
        ticker: &str,
    ) -> sqlx::Result<IntradayPrice> {
        sqlx::query_as::<_, IntradayPrice>(
            "SELECT intraday_prices.id,
             intraday_prices.stock_id,
             ROUND(intraday_prices.price::NUMERIC,2)::DOUBLE PRECISION as price,
             intraday_prices.volume,
             intraday_prices.timestamp,
             stocks.ticker
         FROM intraday_prices
         JOIN stocks ON stocks.id = intraday_prices.stock_id AND stocks.ticker = $1
         ORDER BY intraday_prices.timestamp DESC
         LIMIT 1",
        )
            .bind(ticker)
            .fetch_one(conn).await
    }

    pub async fn insert(
        conn: &crate::db::DbPool,
        other_stock_ticker: &str,
        other_price: f64,
        other_volume: i64,
        other_timestamp: chrono::NaiveDateTime,
    ) -> sqlx::Result<sqlx::postgres::PgDone> {
        sqlx::query("INSERT INTO intraday_prices VALUES
        (DEFAULT, (SELECT id FROM stocks WHERE ticker = $1 LIMIT 1), $2, $3, $4)")
            .bind(other_stock_ticker)
            .bind(other_price)
            .bind(other_volume)
            .bind(other_timestamp)
            .execute(conn)
            .await
    }

    pub async fn get_rsi_by_tickers(
        conn: &crate::db::DbPool,
        tickers: &Vec<String>,
    ) -> Vec<f64> {
        let price_queries = tickers
            .iter()
            .map(|ticker| IntradayPrice::get_rsi_by_ticker(conn, ticker));
        let query_results = futures::future::join_all(price_queries).await;
        let (oks, errs): (Vec<_>, Vec<_>) = query_results
            .into_iter()
            .partition_map(|r| match r {
                Ok(v) => itertools::Either::Left(v),
                Err(v) => itertools::Either::Right(v),
            });
        errs.iter().for_each(|x| log::error!("get_rsi_by_tickers(): Failed to find ticker: {}", x));
        oks
    }

    pub async fn get_rsi_by_ticker(
        conn: &crate::db::DbPool,
        ticker: &str) -> sqlx::Result<f64> {
        let rsi_interval = 14;

        #[derive(sqlx::FromRow, Debug)]
        pub struct Price(f64);

        let price_structs = sqlx::query_as::<_, Price>(
            "SELECT price
         FROM intraday_prices
         JOIN stocks ON stocks.id = intraday_prices.stock_id AND stocks.ticker = $1
         ORDER BY timestamp DESC
         LIMIT 15",
        )
            .bind(ticker)
            .fetch_all(conn)
            .await?;

        let latest_15 = price_structs
            .iter()
            .map(|p| p.0)
            .collect::<Vec<f64>>();
        //
        let mut up_price_bars: Vec<f64> = vec!();
        let mut down_price_bars: Vec<f64> = vec!();

        for (i, p) in latest_15.iter().enumerate() {
            if i == rsi_interval as usize {
                break;
            }
            let curr = p;
            let next = latest_15[i + 1];
            let price_bar = next - curr;
            if price_bar < 0.0 {
                down_price_bars.push(price_bar);
            } else {
                up_price_bars.push(price_bar);
            }
        }
        let down_sum: f64 = Iterator::sum(down_price_bars.iter());
        let average_down = f64::abs(down_sum / (down_price_bars.len() as f64));

        let up_sum: f64 = Iterator::sum(up_price_bars.iter());
        let average_up = f64::abs(up_sum / (up_price_bars.len() as f64));

        Ok(f64::from(1) -
            f64::from(1) /
                (f64::from(1) + (average_up / average_down)))
    }

    // fn mean(list: &[i32]) -> f64 {
    //     let sum: i32 = Iterator::sum(list.iter());
    //     f64::from(sum) / (list.len() as f64)
    // }
}
