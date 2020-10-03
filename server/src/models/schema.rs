table! {
    client_subscriptions (client_subscription_id) {
        client_subscription_id -> Integer,
        client_id -> Integer,
        stock_id -> Integer,
        created_at -> Timestamp,
    }
}

table! {
    clients (id) {
        id -> Integer,
        endpoint -> Text,
        p256dh -> Text,
        auth -> Text,
        created_at -> Timestamp,
    }
}

table! {
    intraday_prices (id) {
        id -> Integer,
        stock_id -> Integer,
        price -> Double,
        volume -> BigInt,
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

joinable!(client_subscriptions -> clients (client_id));
joinable!(client_subscriptions -> stocks (stock_id));
joinable!(intraday_prices -> stocks (stock_id));

allow_tables_to_appear_in_same_query!(
    client_subscriptions,
    clients,
    intraday_prices,
    stocks,
    users,
);
