use actix::prelude::*;
use log::{info, warn};
use chrono::{Utc, TimeZone};
use sqlx::{Transaction, Error, Postgres};

pub(crate) struct OptionsActor {
    pub(crate) pool: crate::db::DbPool,
    pub(crate) stock_list: crate::StockPool,
}

impl Actor for OptionsActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let conn = self.pool.to_owned();
        async move {
            let mut interval = actix_web::rt::time::interval(std::time::Duration::from_secs(60));
            loop {
                let tickers = match crate::models::Stock::get_unique_tickers(&conn).await {
                    Ok(v) => v,
                    Err(_) => vec![]
                };
                if super::is_open_market_hours(chrono::Utc::now()) {
                    match fetch_options(&conn, &tickers).await {
                        Ok(_) => info!("Fetched all options quotes"),
                        Err(e) => warn!("Failed to fetch option quotes from TD, {:?}", e),
                    }
                }
                interval.tick().await;
            }
        }
            .into_actor(self)
            .spawn(ctx);
    }
}

pub async fn fetch_options(conn: &crate::db::DbPool,
                           tickers: &[String]) -> actix_web::Result<(), actix_web::Error> {
    use crate::models::{TDOptionChain, OptionType};
    let client = actix_web::client::Client::default();
    for ticker in tickers {
        let mut txn = match conn.begin().await  {
            Ok(x) => x,
            Err(_) => continue
        };

        sqlx::query("DELETE FROM option_quotes
            WHERE stock_id = (SELECT id from stocks WHERE ticker = $1)"
        ).bind(ticker)
            .execute(&mut txn)
            .await;

        let url = format!("https://api.tdameritrade.com/v1/marketdata/chains?apikey=YPUACAREWAHFTZDFPJJ0FKWN8B7NVVHF&symbol={}", ticker);
        let mut response = client.get(url).send().await?;
        let body = response.body().limit(50 * (1 << 20)).await?; //50MB limit
        let option_chain: TDOptionChain = serde_json::from_slice(&body)?;

        for option_iter in vec![
            (option_chain.call_exp_date_map, OptionType::Call),
            (option_chain.put_exp_date_map, OptionType::Put)] {
            for (_expiry_date, strike_map) in option_iter.0 {
                for (_strike, option_quotes) in strike_map {
                    for option_quote in option_quotes {
                        let res = sqlx::query("INSERT INTO option_quotes VALUES
        (DEFAULT, $1, (SELECT id FROM stocks WHERE ticker = $2 LIMIT 1), $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)")
                            .bind(&option_quote.symbol)
                            .bind(&option_chain.symbol)
                            .bind(&option_iter.1)
                            .bind(option_quote.strike_price)
                            .bind(Utc.timestamp_millis(option_quote.expiration_date))
                            .bind(option_quote.bid)
                            .bind(option_quote.ask)
                            .bind(option_quote.last)
                            .bind(option_quote.delta.as_f64())
                            .bind(option_quote.gamma.as_f64())
                            .bind(option_quote.theta.as_f64())
                            .bind(option_quote.vega.as_f64())
                            .bind(option_quote.rho.as_f64())
                            .bind(option_quote.volatility.as_f64())
                            .bind(option_quote.time_value)
                            .execute(&mut txn)
                            .await;
                        match res {
                            Err(e) => log::error!("failed to insert option data: {}", e),
                            _ => ()
                        }
                    }
                }
            }
        }
        match txn.commit().await {
            Err(e) => log::error!("failed to commit for ticker {}: {}", ticker, e),
            _ => ()
        }
    }
    Ok(())
}
