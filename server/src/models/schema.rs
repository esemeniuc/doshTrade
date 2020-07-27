table! {
    client_subscriptions (client_subscription_id) {
        client_subscription_id -> Integer,
        client_id -> Integer,
        stock_id -> Integer,
    }
}

table! {
    clients (id) {
        id -> Integer,
        endpoint -> Text,
        p256dh -> Text,
        auth -> Text,
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

table! {
    events (id) {
        id -> Integer,
        url -> Text,
        ip -> Text,
        user_agent -> Text,
        fingerprint -> Text,
        is_private -> Bool,
        property_id -> Text,
        created_at -> Timestamp,
    }
}

table! {
    properties (id) {
        id -> Text,
        website_name -> Text,
        website_url -> Text,
        user_id -> Integer,
        created_at -> Timestamp,
    }
}

joinable!(events -> properties (property_id));
joinable!(properties -> users (user_id));

joinable!(client_subscriptions -> clients (client_id));
joinable!(client_subscriptions -> stocks (stock_id));
joinable!(intraday_prices -> stocks (stock_id));

allow_tables_to_appear_in_same_query!(
    client_subscriptions,
    clients,
    intraday_prices,
    stocks,
    users,
    events,
    properties,
);
