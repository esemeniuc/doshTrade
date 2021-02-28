use std::collections::HashMap;

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
    pub vega: ::serde_json::Value,
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

#[derive(async_graphql::SimpleObject, sqlx::FromRow, Clone)]
pub struct OptionQuote {
    // TODO make optionID and also add identifier on Postgres and POP
    pub string_id: String,
    pub option_type: OptionType,
    pub strike: Option<f64>,
    pub expiration: String,
    pub days_to_expiration: String,
    pub bid: Option<f64>,
    pub ask: Option<f64>,
    pub last: Option<f64>,
    pub delta: f64,
    pub gamma: f64,
    pub theta: f64,
    pub vega: f64,
    pub rho: f64,
    pub volatility: f64,
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
}
