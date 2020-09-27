use diesel::prelude::*;

use crate::models::schema::client_subscriptions;
use crate::models::schema::client_subscriptions::dsl::*;

#[derive(Identifiable, Queryable, Debug)]
pub struct ClientSubscription {
    pub id: i32,
    pub client_id: i32,
    pub stock_id: i32,
    pub created_at: chrono::NaiveDateTime,
}

impl ClientSubscription {
    pub fn insert(
        conn: &crate::db::DbPoolConn,
        other_client_id: i32,
        other_stock_id: i32,
    ) -> QueryResult<usize> {
        diesel::insert_into(client_subscriptions::table)
            .values((client_id.eq(other_client_id), stock_id.eq(other_stock_id)))
            .execute(conn)
    }
}
