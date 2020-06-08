use diesel::prelude::*;
use crate::models::property::Property;
use crate::models::schema::events;
use crate::models::schema::events::dsl::*;

#[derive(Identifiable, Queryable, Associations, Debug, juniper::GraphQLObject)]
#[belongs_to(Property)]
pub struct Event {
    pub id: i32,
    pub url: String,
    pub ip: String,
    pub user_agent: String,
    pub fingerprint: String,
    pub is_private: bool,
    pub property_id: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, QueryableByName, juniper::GraphQLObject)]
pub struct PrivateCounts {
    #[sql_type = "diesel::sql_types::Text"]
    pub date: String,
    #[sql_type = "diesel::sql_types::Integer"]
    pub private_count: i32,
    #[sql_type = "diesel::sql_types::Integer"]
    pub non_private_count: i32,
}

impl Event {
    pub fn get_private_stats(conn: &crate::db::DbPoolConn, other_property_id: &str) -> QueryResult<Vec<PrivateCounts>> {
        diesel::sql_query(r#"
WITH recursive
    date_range(start_date, end_date) AS (
        SELECT MIN(date(created_at)), MAX(date(created_at))
        FROM events
        WHERE property_id = ?1
    ),
    all_dates(day) AS (
        -- anchor
        SELECT start_date
        FROM date_range
        UNION ALL
        -- recursion with stop condition
        SELECT date(day, "+1 days") AS new_date
        FROM all_dates,
             date_range
        WHERE new_date <= date_range.end_date
    ),
    private_events(event_date, count) AS (
        SELECT date(created_at), COUNT(*)
        FROM events
        WHERE is_private = true
          AND property_id = ?1
        GROUP BY date(created_at)
-- outputs:
-- 2020-05-06|11
    ),
    non_private_events(event_date, count) AS (
        SELECT date(created_at), COUNT(*)
        FROM events
        WHERE is_private = false
          AND property_id = ?1
        GROUP BY date(created_at)
-- outputs:
-- 2020-05-06|8
-- 2020-05-07|5
    )
SELECT all_dates.day                         AS date,
       COALESCE(private_events.count, 0)     AS private_count,
       COALESCE(non_private_events.count, 0) AS non_private_count
FROM all_dates
         LEFT JOIN private_events ON private_events.event_date = all_dates.day
         LEFT JOIN non_private_events ON non_private_events.event_date = all_dates.day;"#)
            .bind::<diesel::sql_types::Text, _>(other_property_id)
            .get_results(conn)
    }

    pub fn insert(conn: &crate::db::DbPoolConn,
                  other_url: &str,
                  other_ip: &str,
                  other_user_agent: &str,
                  other_fingerprint: &str,
                  other_is_private: bool,
                  other_property_id: &str, ) -> QueryResult<usize> {
        diesel::insert_into(events::table).values((
            url.eq(other_url),
            ip.eq(other_ip),
            user_agent.eq(other_user_agent),
            fingerprint.eq(other_fingerprint),
            is_private.eq(other_is_private),
            property_id.eq(other_property_id),
            created_at.eq(chrono::Local::now().naive_utc()))).execute(conn)
    }

    pub fn get_private_stats_by_date(conn: &crate::db::DbPoolConn,
                                     other_property_id: &str,
                                     start_date: &chrono::NaiveDate,
                                     end_date: &chrono::NaiveDate) -> QueryResult<Vec<PrivateCounts>> {
        let query = diesel::sql_query(r#"
WITH recursive
    date_range(start_date, end_date) AS (
        SELECT ?2, ?3
    ),
    all_dates(day) AS (
        -- anchor
        SELECT start_date
        FROM date_range
        UNION ALL
        -- recursion with stop condition
        SELECT date(day, "+1 days") AS new_date
        FROM all_dates,
             date_range
        WHERE new_date <= date_range.end_date
    ),
    private_events(event_date, count) AS (
        SELECT date(created_at), COUNT(*)
        FROM events
        WHERE is_private = true
          AND property_id = ?1
        GROUP BY date(created_at)
-- outputs:
-- 2020-05-06|11
    ),
    non_private_events(event_date, count) AS (
        SELECT date(created_at), COUNT(*)
        FROM events
        WHERE is_private = false
          AND property_id = ?1
        GROUP BY date(created_at)
-- outputs:
-- 2020-05-06|8
-- 2020-05-07|5
    )
SELECT all_dates.day                         AS date,
       COALESCE(private_events.count, 0)     AS private_count,
       COALESCE(non_private_events.count, 0) AS non_private_count
FROM all_dates
         LEFT JOIN private_events ON private_events.event_date = all_dates.day
         LEFT JOIN non_private_events ON non_private_events.event_date = all_dates.day;"#)
            .bind::<diesel::sql_types::Text, _>(other_property_id)
            .bind::<diesel::sql_types::Date, _>(start_date)
            .bind::<diesel::sql_types::Date, _>(end_date);
        let sql = diesel::debug_query::<diesel::sqlite::Sqlite, _>(&query).to_string();
        println!("{}", sql);
        query.get_results(conn)
    }
}

