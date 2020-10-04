use diesel::prelude::*;

use crate::models::schema::clients;
use crate::models::schema::clients::dsl::*;
use crate::push_notification::PushSubscription;

#[derive(Identifiable, Queryable, Debug)]
pub struct Client {
    pub id: i32,
    pub endpoint: String,
    pub p256dh: String,
    pub auth: String,
    pub created_at: chrono::NaiveDateTime,
}

impl Client {
    pub fn find(conn: &crate::db::DbPoolConn, client: &PushSubscription) -> QueryResult<Client> {
        clients
            .filter(endpoint.eq(&client.endpoint))
            .filter(p256dh.eq(&client.keys.p256dh))
            .filter(auth.eq(&client.keys.auth))
            .first::<Client>(conn)
    }

    pub fn upsert(
        conn: &crate::db::DbPoolConn,
        client: &crate::push_notification::PushSubscription,
    ) -> QueryResult<Client> {
        let query = clients
            .filter(endpoint.eq(client.endpoint.to_owned()))
            .filter(p256dh.eq(client.keys.p256dh.to_owned()))
            .filter(auth.eq(client.keys.auth.to_owned()));

        if query.clone().count().get_result(conn) == Ok(0) {
            return diesel::insert_into(clients)
                .values((
                    endpoint.eq(client.endpoint.to_owned()),
                    p256dh.eq(client.keys.p256dh.to_owned()),
                    auth.eq(client.keys.auth.to_owned()),
                    created_at.eq(chrono::Local::now().naive_utc()),
                ))
                .execute(conn)
                .and_then(|_| query.first::<Client>(conn));
        }

        query.first::<Client>(conn)
    }
}
