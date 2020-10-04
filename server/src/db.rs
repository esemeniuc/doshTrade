use diesel::connection::SimpleConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;
pub type DbPoolConn = PooledConnection<ConnectionManager<SqliteConnection>>;

pub fn establish_connection(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

#[allow(dead_code)]
pub fn establish_connection_temp_db() -> DbPool {
    let manager = ConnectionManager::<SqliteConnection>::new(":memory:");

    #[derive(Debug)]
    struct SqliteForeignKey {}
    impl diesel::r2d2::CustomizeConnection<SqliteConnection, diesel::r2d2::Error> for SqliteForeignKey {
        fn on_acquire(&self, conn: &mut SqliteConnection) -> Result<(), diesel::r2d2::Error> {
            conn.batch_execute("PRAGMA foreign_keys = ON")
                .map_err(diesel::r2d2::Error::QueryError)
        }
    }

    Pool::builder()
        .connection_customizer(Box::new(SqliteForeignKey {}))
        .build(manager)
        .expect("Failed to create pool.")
}

pub fn run_migrations(conn: &DbPoolConn) -> Result<(), diesel_migrations::RunMigrationsError> {
    embed_migrations!("migrations");
    // This will run the necessary migrations.
    // embedded_migrations::run(&pool.get().unwrap());

    // By default the output is thrown out. If you want to redirect it to stdout, you
    // should call embedded_migrations::run_with_output.
    embedded_migrations::run_with_output(conn, &mut std::io::stdout())
}

pub fn seed(conn: &DbPoolConn) -> QueryResult<usize> {
    use crate::models::schema::stocks::dsl::*;
    //TODO don't insert every time on startup
    // let stocks_list = vec![
    //     (ticker.eq("NFLX"), name.eq("Netflix")),
    //     (ticker.eq("AAPL"), name.eq("Apple")),
    //     (ticker.eq("GOOG"), name.eq("Google")),
    // ];
    //cant insert vec: https://github.com/diesel-rs/diesel/issues/2258#issuecomment-569809673

    diesel::insert_into(stocks)
        .values((ticker.eq("AAPL"), name.eq("Apple")))
        .execute(conn)
        .and_then(|_| {
            diesel::insert_into(stocks)
                .values((ticker.eq("GOOG"), name.eq("Google")))
                .execute(conn)
        })
        .and_then(|_| {
            diesel::insert_into(stocks)
                .values((ticker.eq("NFLX"), name.eq("Netflix")))
                .execute(conn)
        })
}

#[allow(dead_code)]
fn undo_migrations(conn: &DbPoolConn) {
    loop {
        if let Err(_) = diesel_migrations::revert_latest_migration(conn) {
            break;
        }
    }
}

#[allow(dead_code)]
pub fn reset_db(conn: &DbPoolConn) -> () {
    undo_migrations(conn);
    run_migrations(conn).expect("failed to run migration");
}
