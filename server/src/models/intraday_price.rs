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
        errs.iter().for_each(|x| log::warn!("get_latest_by_tickers(): Failed to find ticker: {}", x));
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
        rsi_interval: u32,
    ) -> Vec<f64> {
        let price_queries = tickers
            .iter()
            .map(|ticker| IntradayPrice::get_rsi_by_ticker(conn, ticker, rsi_interval));
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

    pub async fn get_rsi_by_stock_id(
        conn: &crate::db::DbPool,
        stock_id: i32,
        rsi_interval: u32) -> sqlx::Result<f64> {
        let latest_n: Vec<f64> = sqlx::query_scalar(
            "SELECT price
         FROM intraday_prices
         WHERE stock_id = $1
         ORDER BY timestamp DESC
         LIMIT $2",
        )
            .bind(stock_id)
            .bind(rsi_interval)
            .fetch_all(conn)
            .await?;
        Ok(IntradayPrice::calc_rsi(latest_n))
    }

    pub async fn get_rsi_by_ticker(
        conn: &crate::db::DbPool,
        ticker: &str,
        rsi_interval: u32) -> sqlx::Result<f64> {
        let latest_n: Vec<f64> = sqlx::query_scalar(
            "SELECT price
         FROM intraday_prices
         JOIN stocks ON stocks.id = intraday_prices.stock_id AND stocks.ticker = $1
         ORDER BY timestamp DESC
         LIMIT $2",
        )
            .bind(ticker)
            .bind(rsi_interval)
            .fetch_all(conn)
            .await?;

        Ok(IntradayPrice::calc_rsi(latest_n))
    }

    pub fn calc_rsi(latest_n_prices: Vec<f64>) -> f64 {
        if latest_n_prices.len() < 2 {
            return 0.0;
        }

        let mut up_price_bars: Vec<f64> = vec!();
        let mut down_price_bars: Vec<f64> = vec!();

        for i in 1..latest_n_prices.len() {
            let prev = latest_n_prices[i - 1];
            let curr = latest_n_prices[i];
            let price_bar = curr - prev;
            if price_bar < 0.0 {
                down_price_bars.push(price_bar);
            } else {
                up_price_bars.push(price_bar);
            }
        }
        // In the case that price does not go up or down, return middle value,
        if down_price_bars.len() + up_price_bars.len() == 0 {
            return 0.5;
        }
        // In the case that price does not go down at all, return maximal value,
        if down_price_bars.len() == 0 {
            return 1.0;
        }
        // In the case that price does not go up at all, return minimal value,
        if up_price_bars.len() == 0 {
            return 0.0;
        }
        let down_sum: f64 = Iterator::sum(down_price_bars.iter());
        let average_down = f64::abs(down_sum / down_price_bars.len() as f64);

        let up_sum: f64 = Iterator::sum(up_price_bars.iter());
        let average_up = f64::abs(up_sum / (up_price_bars.len() as f64));

        1.0 - (1.0 / (1.0 + (average_up / average_down)))
    }
}
