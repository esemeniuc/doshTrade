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
}

impl Client {
    pub fn find(conn: &crate::db::DbPoolConn, client: &PushSubscription) -> QueryResult<Client> {
        clients
            .filter(endpoint.eq(&client.endpoint))
            .filter(p256dh.eq(&client.keys.p256dh))
            .filter(auth.eq(&client.keys.auth))
            .first::<Client>(conn)
    }

    pub fn insert(
        conn: &crate::db::DbPoolConn,
        client: &crate::push_notification::PushSubscription,
    ) -> QueryResult<Client> {
        diesel::insert_into(clients::table)
            .values((
                endpoint.eq(client.endpoint.to_owned()),
                p256dh.eq(client.keys.p256dh.to_owned()),
                auth.eq(client.keys.auth.to_owned()),
            ))
            .execute(conn)
            .and_then(|_| {
                clients
                    .filter(endpoint.eq(client.endpoint.to_owned()))
                    .filter(p256dh.eq(client.keys.p256dh.to_owned()))
                    .filter(auth.eq(client.keys.auth.to_owned()))
                    .first::<Client>(conn)
            })
    }
}
