table! {
    intraday_prices (id) {
        id -> Text,
        stock_id -> Text,
        price -> Text,
        volume -> Integer,
        timestamp -> Timestamp,
    }
}

table! {
    stocks (id) {
        id -> Integer,
        ticker -> Text,
        name -> Text,
    }
}

table! {
    users (id) {
        id -> Integer,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        password -> Text,
        auth_bearer_token -> Text,
        created_at -> Timestamp,
    }
}

joinable!(intraday_prices -> stocks (stock_id));

allow_tables_to_appear_in_same_query!(
    intraday_prices,
    stocks,
    users,
);
