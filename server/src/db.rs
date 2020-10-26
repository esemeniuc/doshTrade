pub type DbPool = sqlx::SqlitePool;
pub type DbPoolConn = sqlx::SqlitePool;

pub async fn seed(conn: &DbPoolConn) -> sqlx::Result<sqlx::sqlite::SqliteDone> {
    // Read migrations from a local folder: ./migrations
    // let m = sqlx::migrate::Migrator::new(std::path::Path::new("./migrations")).await?;
    // m.run(conn).await?;
    sqlx::migrate!("./migrations").run(conn).await?;

    //TODO don't insert every time on startup
    let stocks_list = vec![
        ("AAPL", "Apple"),
        ("GOOG", "Google"),
        ("GE", "General Electric"),
        ("NFLX", "Netflix"),
    ];

    let inserts = stocks_list.iter().map(|stock| {
        sqlx::query("INSERT INTO stocks VALUES (null, ?, ?)")
            .bind(stock.0)
            .bind(stock.1)
            .execute(conn)
    });
    //TODO check insert status
    futures::future::join_all(inserts).await.into_iter().fold(
        sqlx::Result::Ok(sqlx::sqlite::SqliteDone::default()),
        |acc, curr| {
            if Result::is_ok(&curr) {
                acc
            } else {
                curr
            }
        },
    )
}
