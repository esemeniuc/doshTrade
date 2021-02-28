use std::time::SystemTime;
use actix::prelude::*;

use log::{error, info, trace, warn};

use crate::models::IntradayPrice;

pub(crate) struct StockActor {
    pub(crate) pool: crate::db::DbPool,
    pub(crate) stock_list: crate::StockPool,
}

impl Actor for StockActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let conn = self.pool.to_owned();

        async move {
            let mut interval = actix_web::rt::time::interval(std::time::Duration::from_secs(30));
            loop {
                //example
                // userA, [A,B,C] -> 3 rows in db
                // userB, [B,C] -> 2 rows in db

                //select distinct(stockticker) from subscriptions
                //returns [A,B,C]

                //call td with this
                //insert price data into intraday_prices table

                //periodically scan subscriptions table
                //calculate if a watched ticker should notify based on each row
                //
                //do calculation using intraday_prices table
                //eg userA with stock ticker B ($$$TICKER)
                //db: select the intraday updates needed for calculation
                // SELECT price, volume FROM intraday_updates
                // JOIN stocks ON stocks.id = intraday_prices.id
                // WHERE stocks.ticker = $$$TICKER
                // ORDER by timestamp DESC
                // LIMIT 5 (whatever is actually necessary for calc)
                let tickers = match crate::models::Stock::get_unique_tickers(&conn).await{
                    Ok(v) =>v,
                    Err(_) => vec![]
                };

                if super::is_open_market_hours(chrono::Utc::now()) {
                    match fetch_and_insert(&conn, &tickers).await {
                        Ok(_) => info!("Fetched all tickers"),
                        Err(e) => warn!("Failed to fetch tickers from TD, {:?}", e),
                    }
                }
                interval.tick().await;
            }
        }
            .into_actor(self)
            .spawn(ctx);
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockQuote {
    pub asset_type: String,
    pub asset_main_type: String,
    pub cusip: String,
    pub symbol: String,
    pub description: String,
    pub bid_price: f64,
    pub bid_size: i64,
    pub bid_id: String,
    pub ask_price: f64,
    pub ask_size: i64,
    pub ask_id: String,
    pub last_price: f64,
    pub last_size: i64,
    pub last_id: String,
    pub open_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub bid_tick: String,
    pub close_price: f64,
    pub net_change: f64,
    pub total_volume: i64,
    pub quote_time_in_long: i64,
    pub trade_time_in_long: i64,
    pub mark: f64,
    pub exchange: String,
    pub exchange_name: String,
    pub marginable: bool,
    pub shortable: bool,
    pub volatility: f64,
    pub digits: i64,
    #[serde(rename = "52WkHigh")]
    pub n52_wk_high: f64,
    #[serde(rename = "52WkLow")]
    pub n52_wk_low: f64,
    #[serde(rename = "nAV")]
    pub n_av: f64,
    pub pe_ratio: f64,
    pub div_amount: f64,
    pub div_yield: f64,
    pub div_date: String,
    pub security_status: String,
    pub regular_market_last_price: f64,
    pub regular_market_last_size: i64,
    pub regular_market_net_change: f64,
    pub regular_market_trade_time_in_long: i64,
    pub net_percent_change_in_double: f64,
    pub mark_change_in_double: f64,
    pub mark_percent_change_in_double: f64,
    pub regular_market_percent_change_in_double: f64,
    pub delayed: bool,
}


pub async fn fetch_and_insert(
    conn: &crate::db::DbPool,
    tickers: &[String],
) -> anyhow::Result<()> {
    info!("Getting updates from TD for {:#?}", tickers);
    trace!(
        "Started at {}",
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );

    let quotes = fetch_quotes(tickers).await?;
    let query_result = IntradayPrice::insert_many(conn, quotes).await;
    if query_result.len() != tickers.len() {
        error!("Failed to fetch intraday update")
    }

    trace!(
        "Ended at {}",
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );
    Ok(())
}

pub async fn fetch_quotes(tickers: &[String]) -> anyhow::Result<Vec<StockQuote>> {
    info!("Fetching tickers: {:?}", tickers);
    let tickers_str = tickers.join(",");
    let url = format!("https://api.tdameritrade.com/v1/marketdata/quotes?apikey=YPUACAREWAHFTZDFPJJ0FKWN8B7NVVHF&symbol={}", tickers_str);
    info!("Using url: {}", url);
    let ticker_to_quotes: std::collections::HashMap<String, StockQuote> = reqwest::get(&url).await?.json().await?;
    Ok(ticker_to_quotes.into_iter().map(|(_k, v)| v).collect::<Vec<StockQuote>>())
}
