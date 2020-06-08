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

joinable!(events -> properties (property_id));
joinable!(properties -> users (user_id));

allow_tables_to_appear_in_same_query!(
    events,
    properties,
    users,
);
