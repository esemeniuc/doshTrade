use diesel::prelude::*;
use diesel::dsl::count;
use crate::models::user::User;
use crate::models::schema::properties;
use crate::models::schema::properties::dsl::*;

#[derive(Identifiable, Queryable, Associations, Debug, juniper::GraphQLObject)]
#[table_name = "properties"]
#[belongs_to(User)]
pub struct Property {
    pub id: String,
    pub website_name: String,
    pub website_url: String,
    pub user_id: i32,
    pub created_at: chrono::NaiveDateTime,
}

impl Property {
    //NOT USED BECAUSE properties.id switched to string type
    // pub fn parse_id(property_id: &str) -> Option<(i32, i32)> {
    //     let id_parts: Vec<Option<i32>> = property_id.split('-').map(|e: &str| e.parse::<i32>().ok()).collect();
    //     match id_parts[..] {
    //         [Some(aa), Some(bb)] => Some((1, 2)),
    //         _ => None
    //     }
    // }

    pub fn generate_property_id_for_user_id(conn: &crate::db::DbPoolConn, other_user_id: i32) -> QueryResult<String> {
        //get number of properties for jwt usr
        //make id like {user_id}-{next_property_number}
        properties.select(count(properties::id))
            .filter(user_id.eq(other_user_id))
            .first(conn)
            .map(|previous_property_number: i64| format!("{}-{}", other_user_id, previous_property_number + 1))
    }

    pub fn is_property_id_belong_to_user_id(conn: &crate::db::DbPoolConn, other_property_id: &str, other_user_id: i32) -> QueryResult<bool> {
        diesel::select(diesel::dsl::exists(properties
            .find(other_property_id)
            .filter(user_id.eq(other_user_id))))
            .get_result(conn)
    }

    pub fn insert(conn: &crate::db::DbPoolConn,
                  other_id: &str,
                  other_website_name: &str,
                  other_website_url: &str,
                  other_user_id: i32) -> QueryResult<Property> {
        diesel::insert_into(properties::table).values((
            id.eq(other_id),
            website_name.eq(other_website_name),
            website_url.eq(other_website_url),
            user_id.eq(other_user_id),
            created_at.eq(chrono::Local::now().naive_utc())))
            .execute(conn)
            .and_then(|_rows_affected: usize| properties
                .find(other_id)
                .get_result::<Property>(conn)
            )
    }

    pub fn get_properties(conn: &crate::db::DbPoolConn, other_user_id: i32) -> QueryResult<Vec<Property>> {
        properties
            .filter(user_id.eq(other_user_id))
            .get_results(conn)
    }
}
