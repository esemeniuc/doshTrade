use itertools::Itertools;
use crate::background_tasks::StockQuote;

#[derive(sqlx::FromRow, Debug)]
pub struct IntradayPrice {
    pub id: i32,
    pub stock_id: i32,
    pub price: f64,
    pub volume: i64,
    pub timestamp: chrono::NaiveDateTime,
    pub ticker: String,
}

// use ta_lib_wrapper::{TA_Integer, TA_Real, TA_RSI, TA_RetCode};

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

    pub async fn insert_many(
        conn: &crate::db::DbPool,
        quotes: Vec<StockQuote>,
    ) {
        let inserts = quotes
            .iter()
            .map(|quote| {
                //NOTE: this should have the time of the last executed trade (not the time of quote). This may change in the future
                let secs = quote.trade_time_in_long / 1000; //time comes in as milliseconds, convert to sec
                let remaining_nanos = (quote.trade_time_in_long % 1000) * 1_000_000;
                IntradayPrice::insert(conn,
                                      &quote.symbol,
                                      quote.last_price,
                                      quote.total_volume,
                                      chrono::NaiveDateTime::from_timestamp(secs, remaining_nanos as u32))
            });
        let query_results = futures::future::join_all(inserts).await;
        let (oks, errs): (Vec<_>, Vec<_>) = query_results
            .into_iter()
            .partition_map(|r| match r {
                Ok(v) => itertools::Either::Left(v),
                Err(v) => itertools::Either::Right(v),
            });
        errs.iter().for_each(|x| log::error!("insert_many(): Failed to insert quotes for: {}", x));
        // oks
    }

    pub async fn insert(
        conn: &crate::db::DbPool,
        stock_ticker: &str,
        price: f64,
        volume: i64,
        timestamp: chrono::NaiveDateTime,
    ) -> sqlx::Result<sqlx::postgres::PgDone> {
        sqlx::query("INSERT INTO intraday_prices VALUES
        (DEFAULT, (SELECT id FROM stocks WHERE ticker = $1 LIMIT 1), $2, $3, $4)")
            .bind(stock_ticker)
            .bind(price)
            .bind(volume)
            .bind(timestamp)
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

    pub async fn get_open_prices_by_stock_ids(
        conn: &crate::db::DbPool,
        stock_info: &Vec<(i32, chrono::NaiveDateTime)>,
    ) -> Vec<f64> {
        let open_price_queries = stock_info
            .iter()
            .map(|stock_info| IntradayPrice::get_open_price_by_stock_id(conn, stock_info.0, stock_info.1));
        let query_results = futures::future::join_all(open_price_queries).await;
        let (oks, errs): (Vec<_>, Vec<_>) = query_results
            .into_iter()
            .partition_map(|r| match r {
                Ok(v) => itertools::Either::Left(v),
                Err(v) => itertools::Either::Right(v),
            });
        errs.iter().for_each(|x| log::error!("get_open_prices_by_stock_ids(): Failed to find: {}", x));
        oks
    }

    pub async fn get_open_price_by_stock_id(
        conn: &crate::db::DbPool,
        stock_id: i32,
        latest_timestamp: chrono::NaiveDateTime) -> sqlx::Result<f64> {
        sqlx::query_scalar(
            "WITH day_prices AS (SELECT *
                       FROM intraday_prices
                       WHERE stock_id = $1
                         AND timestamp >= date($2)
                         AND timestamp < date($2) + 1)
SELECT price
FROM day_prices
WHERE timestamp = (SELECT min(timestamp) FROM day_prices)",
        )
            .bind(stock_id)
            .bind(latest_timestamp)
            .fetch_one(conn)
            .await
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


    // fn rsi(period: u32, close_prices: &Vec<TA_Real>) -> (Vec<TA_Real>, TA_Integer) {
    //     let mut out: Vec<TA_Real> = Vec::with_capacity(close_prices.len());
    //     let mut out_begin: TA_Integer = 0;
    //     let mut out_size: TA_Integer = 0;
    //
    //     unsafe {
    //         let ret_code = TA_RSI(
    //             0,                              // index of the first close to use
    //             close_prices.len() as i32 - 1,  // index of the last close to use
    //             close_prices.as_ptr(),          // pointer to the first element of the vector
    //             period as i32,                  // period of the rsi
    //             &mut out_begin,                 // set to index of the first close to have an rsi value
    //             &mut out_size,                  // set to number of sma values computed
    //             out.as_mut_ptr(),                // pointer to the first element of the output vector
    //         );
    //         match ret_code {
    //             // Indicator was computed correctly, since the vector was filled by TA-lib C library,
    //             // Rust doesn't know what is the new length of the vector, so we set it manually
    //             // to the number of values returned by the TA_RSI call
    //             TA_RetCode::TA_SUCCESS => out.set_len(out_size as usize),
    //             // An error occured
    //             _ => panic!("Could not compute indicator, err: {:?}", ret_code)
    //         }
    //     }
    //
    //     (out, out_begin)
    // }

    // pub fn calc_rsi_rsi_rs(latest_n_prices: Vec<f64>) -> f64 {
    //     use rsi_rs::RSI;
    //     use ta_common::traits::Indicator;
    //     if latest_n_prices.is_empty() {
    //         return 0.0;
    //     }
    //     let mut latest_n_prices = latest_n_prices.clone();
    //     latest_n_prices.reverse();
    //     println!("len = {}", latest_n_prices.len());
    //     let mut rsi = RSI::new(14);
    //     let mut out = None;
    //     for price in latest_n_prices {
    //         out = rsi.next(price);
    //     }
    //     println!("RSI is {}", out.unwrap());
    //     out.unwrap()
    // }

    // pub fn calc_rsi_ta_lib(latest_n_prices: Vec<f64>) -> f64 {
    //     if latest_n_prices.len() < 14 {
    //         return 0.0;
    //     }
    //     let mut latest_n_prices = latest_n_prices.clone();
    //     latest_n_prices.reverse();
    //     let (rsi_values, begin) = IntradayPrice::rsi(14, &latest_n_prices);
    //     for (index, value) in rsi_values.iter().enumerate() {
    //         println!("Close index {} = {}", begin + index as i32 + 1, value);
    //     }
    //     // return rsi_values;
    //     0.0
    // }

    pub fn calc_rsi_june(latest_n_prices: Vec<f64>) -> f64 {
        if latest_n_prices.len() < 2 {
            return 0.0;
        }
        let mut latest_n_prices = latest_n_prices.clone();
        latest_n_prices.reverse();
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

    pub fn calc_rsi(latest_n_prices: Vec<f64>) -> f64 {
        use ta::indicators::RelativeStrengthIndex;
        use ta::Next;
        // let mut latest_n_prices = latest_n_prices.clone();
        // latest_n_prices.reverse();

        if latest_n_prices.is_empty() {
            return 0.0;
        }
        // latest_n_prices.remove(0);
        println!("len = {}", latest_n_prices.len());
        let mut rsi = RelativeStrengthIndex::new(14).unwrap();
        // let out = latest_n_prices.iter().fold((rsi, 0.0),
        //                                       |mut rsi_acc, price| rsi_acc.next(price));

        let mut out = 0.0;
        for price in latest_n_prices {
            out = rsi.next(price);
            println!("RSI is {}", out);
        }
        println!("RSI is {}", out);
        out
    }
}
