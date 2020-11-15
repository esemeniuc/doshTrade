pub type DbPool = sqlx::PgPool;

pub async fn seed(conn: &DbPool) -> sqlx::Result<sqlx::postgres::PgDone> {
    // Read migrations from a local folder: ./migrations
    // let m = sqlx::migrate::Migrator::new(std::path::Path::new("./migrations")).await?;
    // m.run(conn).await?;
    sqlx::migrate!("./migrations").run(conn).await?;

    let stocks_list = vec![
        ("AAPL", "Apple"),
        ("GOOG", "Google"),
        ("GE", "General Electric"),
        ("NFLX", "Netflix"),
    ];

    let inserts = stocks_list.into_iter().map(|stock| {
        sqlx::query("INSERT INTO stocks VALUES (DEFAULT, $1, $2) ON CONFLICT DO NOTHING")
            .bind(stock.0)
            .bind(stock.1)
            .execute(conn)
    });

    futures::future::join_all(inserts).await.into_iter().fold(
        sqlx::Result::Ok(sqlx::postgres::PgDone::default()),
        |acc, curr| {
            if Result::is_ok(&curr) {
                acc
            } else {
                curr
            }
        },
    )
}
