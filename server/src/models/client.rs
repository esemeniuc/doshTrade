use crate::push_notification::PushSubscription;

#[derive(Debug)]
pub struct Client {
    pub id: i32,
    pub endpoint: String,
    pub p256dh: String,
    pub auth: String,
    pub created_at: chrono::NaiveDateTime,
}

impl Client {
    pub async fn upsert(
        conn: &crate::db::DbPoolConn,
        client: &crate::push_notification::PushSubscription,
    ) -> sqlx::Result<i32> {
        let query = sqlx::query("INSERT INTO clients VALUES (null, ?,?,?) ON CONFLICT DO NOTHING")
            .bind(client.endpoint.to_owned())
            .bind(client.keys.p256dh.to_owned())
            .bind(client.keys.auth.to_owned())
            .execute(conn)
            .await?;
        Ok(query.last_insert_rowid() as i32)
    }
}
