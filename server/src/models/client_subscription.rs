
#[derive(Debug)]
pub struct ClientSubscription {
    pub id: i32,
    pub client_id: i32,
    pub stock_id: i32,
    pub created_at: chrono::NaiveDateTime,
}

impl ClientSubscription {
    pub async fn insert(
        conn: &crate::db::DbPoolConn,
        client_id: i32,
        stock_id: i32,
    ) -> sqlx::Result<sqlx::sqlite::SqliteDone> {
        sqlx::query("INSERT INTO client_subscriptions VALUES (null, ?, ?, ?)")
            .bind(client_id)
            .bind(stock_id)
            .bind(chrono::Local::now().naive_utc())
            .execute(conn)
            .await
    }

    pub async fn delete_all(
        conn: &crate::db::DbPoolConn,
        client_id: i32,
    ) -> sqlx::Result<sqlx::sqlite::SqliteDone> {
        sqlx::query("DELETE FROM client_subscriptions WHERE client_id = ?")
            .bind(client_id)
            .execute(conn)
            .await
    }
}
