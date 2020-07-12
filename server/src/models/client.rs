use crate::models::schema::clients;
use crate::models::schema::clients::dsl::*;
use diesel::prelude::*;

#[derive(Identifiable, Queryable, Debug)]
pub struct Client {
    pub id: i32,
    pub endpoint: String,
    pub p256dh: String,
    pub auth: String,
    pub created_at: chrono::NaiveDateTime,
}

impl Client {
    pub fn insert(
        conn: &crate::db::DbPoolConn,
        client: crate::push_notification::PushSubscription,
    ) -> QueryResult<usize> {
        diesel::insert_into(clients::table)
            .values((
                endpoint.eq(client.endpoint),
                p256dh.eq(client.keys.p256dh),
                auth.eq(client.keys.auth),
            ))
            .execute(conn)
    }
}
