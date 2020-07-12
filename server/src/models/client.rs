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
        other_endpoint: &str,
        other_p256dh: &str,
        other_auth: &str,
    ) -> QueryResult<usize> {
        diesel::insert_into(clients::table)
            .values((
                endpoint.eq(other_endpoint),
                p256dh.eq(other_p256dh),
                auth.eq(other_auth),
            ))
            .execute(conn)
    }
}
