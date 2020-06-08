use diesel::prelude::*;
use super::schema::users;
use crate::models::schema::users::dsl::*;
use crate::auth;

#[derive(Identifiable, Queryable, Debug, juniper::GraphQLObject)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub auth_bearer_token: String,
    pub created_at: chrono::NaiveDateTime,
}

impl User {
    //creates (or updates if already exists) the users' auth bearer token for logging in
    //returns none if email/password do not match
    pub fn login(conn: &crate::db::DbPoolConn, input_email: &str, input_password: &str) -> QueryResult<User> {
        users
            .filter(email.eq(input_email))
            .filter(password.eq(input_password))
            .first::<User>(conn)
            .and_then(|user: User| Self::update_token(conn, user.id))
    }

    //returns None if user not found
    fn update_token(conn: &crate::db::DbPoolConn, user_id: i32) -> QueryResult<User> {
        let jwt = crate::auth::generate_bearer_token_now(user_id);
        println!("updated token: {}", jwt);
        diesel::update(users.find(user_id))
            .set(auth_bearer_token.eq(jwt))
            .execute(conn)
            .and_then(|_rows_affected: usize| users
                .find(user_id)
                .first(conn))
    }

    // looks up a user based on their JWT
    pub fn from_jwt(conn: &crate::db::DbPoolConn, jwt: &str) -> QueryResult<User> {
        match auth::get_user_id(&jwt) {
            Ok(user_id) => users.find(user_id).get_result::<User>(conn),
            _ => Err(diesel::result::Error::NotFound),
        }
    }

    pub fn insert(conn: &crate::db::DbPoolConn,
                  other_first_name: &str,
                  other_last_name: &str,
                  other_email: &str,
                  other_password: &str) -> QueryResult<User> {
        let other_created_at = chrono::Local::now().naive_utc();
        diesel::insert_into(users::table) //insert temp record
            .values((first_name.eq(other_first_name),
                     last_name.eq(other_last_name),
                     email.eq(other_email),
                     password.eq(other_password),
                     auth_bearer_token.eq(""), // will be updated later as it needs the user id
                     created_at.eq(other_created_at)))
            .execute(conn)
            .and_then(|_rows_affected: usize| users
                .filter(users::email.eq(other_email))
                .filter(users::password.eq(other_password))
                .first::<User>(conn)) //get temp record for id
            .and_then(|user: User| diesel::update(users.find(user.id)) //update the user record with jwt and embedded user id
                .set(auth_bearer_token.eq(crate::auth::generate_bearer_token(user.id, other_created_at)))
                .execute(conn))
            .and_then(|_rows_affected: usize| users
                .filter(users::email.eq(other_email))
                .first::<User>(conn)) //get final user record
    }

    pub fn is_in_db(conn: &crate::db::DbPoolConn, other_user_id: i32) -> QueryResult<bool> {
        diesel::select(diesel::dsl::exists(users
            .find(other_user_id)))
            .get_result(conn)
    }
}

