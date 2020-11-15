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
        conn: &crate::db::DbPool,
        client: &crate::push_notification::PushSubscription,
    ) -> sqlx::Result<i32> {
        sqlx::query_scalar("WITH insert_result AS (
INSERT
INTO clients
VALUES (DEFAULT,$1,$2,$3,$4)
ON conflict do nothing returning id)
SELECT coalesce(
                   (SELECT * FROM insert_result),
                   (SELECT id FROM clients WHERE endpoint = $1 AND p256dh = $2 AND auth = $3)
           )")
            .bind(client.endpoint.to_owned())
            .bind(client.keys.p256dh.to_owned())
            .bind(client.keys.auth.to_owned())
            .bind(chrono::Local::now().naive_utc())
            .fetch_one(conn)
            .await
    }
}
