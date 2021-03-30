use std::collections::HashMap;
use chrono::{Utc};
use crate::asyncgql::{OptionStrategy, OptionRiskSummary};

#[derive(Default, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TDOptionChain {
    pub symbol: String,
    pub status: String,
    pub underlying: ::serde_json::Value,
    pub strategy: String,
    pub interval: f64,
    pub is_delayed: bool,
    pub is_index: bool,
    pub interest_rate: f64,
    pub underlying_price: f64,
    pub volatility: f64,
    pub days_to_expiration: f64,
    pub number_of_contracts: i64,
    pub put_exp_date_map: HashMap<String, HashMap<String, Vec<TDOptionQuote>>>,
    pub call_exp_date_map: HashMap<String, HashMap<String, Vec<TDOptionQuote>>>,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TDOptionQuote {
    pub put_call: String,
    pub symbol: String,
    pub description: String,
    pub exchange_name: String,
    pub bid: f64,
    pub ask: f64,
    pub last: f64,
    pub mark: f64,
    pub bid_size: i64,
    pub ask_size: i64,
    pub bid_ask_size: String,
    pub last_size: i64,
    pub high_price: f64,
    pub low_price: f64,
    pub open_price: f64,
    pub close_price: f64,
    pub total_volume: i64,
    pub trade_date: ::serde_json::Value,
    pub trade_time_in_long: i64,
    pub quote_time_in_long: i64,
    pub net_change: f64,
    pub volatility: ::serde_json::Value,
    pub delta: ::serde_json::Value,
    pub gamma: ::serde_json::Value,
    pub theta: ::serde_json::Value,
    pub vega: f64,
    pub rho: ::serde_json::Value,
    pub open_interest: i64,
    pub time_value: f64,
    pub theoretical_option_value: ::serde_json::Value,
    pub theoretical_volatility: ::serde_json::Value,
    pub option_deliverables_list: ::serde_json::Value,
    pub strike_price: f64,
    pub expiration_date: i64,
    pub days_to_expiration: i64,
    pub expiration_type: String,
    pub last_trading_day: i64,
    pub multiplier: f64,
    pub settlement_type: String,
    pub deliverable_note: String,
    pub is_index_option: ::serde_json::Value,
    pub percent_change: f64,
    pub mark_change: f64,
    pub mark_percent_change: f64,
    pub in_the_money: bool,
    pub non_standard: bool,
    pub mini: bool,
}

#[derive(sqlx::Type, async_graphql::Enum, Copy, Clone, Eq, PartialEq)]
#[sqlx(rename = "OPTION_TYPE", rename_all = "lowercase")]
pub enum OptionType { Call, Put }

#[derive(sqlx::FromRow, Clone)]
pub struct OptionQuote {
    pub string_id: String,
    pub option_type: OptionType,
    pub strike: Option<f64>,
    pub expiration: String,
    pub days_to_expiration: i32,
    pub bid: Option<f64>,
    pub ask: Option<f64>,
    pub last: Option<f64>,
    pub delta: Option<f64>,
    pub gamma: Option<f64>,
    pub theta: Option<f64>,
    pub vega: f64,
    pub rho: Option<f64>,
    pub volatility: Option<f64>,
    pub time_value: f64,
}

impl OptionQuote {
    pub async fn get_latest_by_ticker(
        conn: &crate::db::DbPool,
        ticker: String,
    ) -> sqlx::Result<Vec<OptionQuote>> {
        sqlx::query_as::<_, OptionQuote>(
            "SELECT
                 string_id,
                 option_type,
                 strike,
                 CAST(expiration AS VARCHAR),
                 EXTRACT(DAY FROM expiration - now())
                 bid,
                 ask,
                 last,
                 delta,
                 gamma,
                 theta,
                 vega,
                 rho,
                 volatility,
                 time_value

         FROM option_quotes
         JOIN stocks ON stocks.id = option_quotes.stock_id AND stocks.ticker = $1
         ORDER BY expiration, strike ASC",
        )
            .bind(ticker)
            .fetch_all(conn).await
    }

    pub async fn get_available_expirations(
        conn: &crate::db::DbPool,
        ticker: &String,
    ) -> sqlx::Result<Vec<String>> {
        sqlx::query_scalar(
            "select distinct CAST(expiration::timestamp AS VARCHAR) from option_quotes
            WHERE stock_id = (SELECT id from stocks WHERE ticker = $1)
            order by expiration asc"
        ).bind(ticker)
            .fetch_all(conn).await
    }

    pub async fn get_option_chain(
        conn: &crate::db::DbPool,
        ticker: String,
        expiration: chrono::DateTime::<Utc>,
        option_type: OptionType,
    ) -> sqlx::Result<Vec<OptionQuote>> {
        sqlx::query_as::<_, OptionQuote>(
            "SELECT
        string_id,
        option_type,
        strike,
        expiration::VARCHAR,
        EXTRACT(DAY FROM expiration - now())::INTEGER AS days_to_expiration,
        bid,
        ask,
        last,
        delta,
        gamma,
        theta,
        vega,
        rho,
        volatility,
        time_value

         FROM option_quotes
         WHERE stock_id = (SELECT id from stocks WHERE ticker = $1)
         AND expiration = $2
         AND option_type = $3
         ORDER BY strike ASC",
        )
            .bind(ticker)
            .bind(expiration)
            .bind(option_type)
            .fetch_all(conn).await
    }

    pub async fn get_risk_summary(
        conn: &crate::db::DbPool,
        option_id: String,
        strategy: OptionStrategy,
    ) -> sqlx::Result<OptionRiskSummary> {
        let (last_option_price, strike_price) = sqlx::query_as::<_, (f64, f64)>(
            "SELECT last, strike
         FROM option_quotes
         WHERE string_id = $1",
        )
            .bind(&option_id)
            .fetch_one(conn).await?;

        let ticker = match option_id.split('_').collect::<Vec<_>>().first() {
            None => return sqlx::Result::Err(sqlx::Error::RowNotFound),
            Some(v) => *v,
        };
        let stock_price = crate::models::IntradayPrice::get_latest_by_ticker(conn, ticker).await?;
        let (max_risk, max_profit, breakeven_at_expiration) = OptionQuote::calc_risk_summary(strategy, stock_price.price, last_option_price, strike_price);

        Ok(OptionRiskSummary {
            max_risk: format!("${}", max_risk * 100.0),
            max_profit: format!("${}", max_profit * 100.0),
            breakeven_at_expiration: format!("${}", breakeven_at_expiration * 100.0),
        })
    }

    //returns max_risk, max_profit, stock_breakeven_at_expiration
    pub fn calc_risk_summary(strategy: OptionStrategy, last_stock_price: f64, last_option_price: f64, strike_price: f64) -> (f64, f64, f64) {
        match strategy {
            OptionStrategy::BuyCall =>
                (last_option_price,
                 f64::INFINITY,
                 strike_price + last_option_price),
            OptionStrategy::BuyPut =>
                (last_option_price,
                 strike_price - last_option_price,
                 strike_price - last_option_price),
            OptionStrategy::SellCall =>
                (f64::INFINITY,
                 last_option_price,
                 strike_price + last_option_price),
            OptionStrategy::SellPut =>
                (strike_price - last_option_price,
                 last_option_price,
                 strike_price - last_option_price)
        }
    }

    //volatility is a percentage value from 0 to 2000ish (not normalized in db)
    //from https://www.optionstrategist.com/calculators/probability
    pub fn calc_pop(stock_price: f64, target_price: f64, days_to_exp: f64, volatility: f64) -> (f64, f64) {
        let p = stock_price;
        let q = target_price;
        let t = days_to_exp / 365.0;
        let v = volatility / 100.0;

        let vt = v * t.sqrt();
        let lnpq = (q / p).ln();
        let d1 = lnpq / vt;

        let y = (1.0 / (1.0 + 0.2316419 * d1.abs()) * 100000.0).floor() / 100000.0;
        let z = (0.3989423 * (-((d1 * d1) / 2.0)).exp() * 100000.0).floor() / 100000.0;
        let y5 = 1.330274 * y.powi(5);
        let y4 = 1.821256 * y.powi(4);
        let y3 = 1.781478 * y.powi(3);
        let y2 = 0.356538 * y.powi(2);
        let y1 = 0.3193815 * y;
        let x = 1.0 - z * (y5 - y4 + y3 - y2 + y1);
        let mut x = (x * 100000.0).floor() / 100000.0;

        if d1 < 0.0 {
            x = 1.0 - x;
        };

        let pbelow = (x * 1000.0).floor() / 10.0;
        let pabove = ((1.0 - x) * 1000.0).floor() / 10.0;

        (pbelow, pabove)
    }
}
