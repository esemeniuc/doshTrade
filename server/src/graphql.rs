use juniper::{FieldResult, RootNode};
use std::ops::Deref;
use crate::models::{User, Event, Property};
use crate::auth;
use crate::models::event::PrivateCounts;

//file based on https://github.com/actix/examples/tree/master/juniper
pub struct Context {
    pub db_conn: crate::db::DbPoolConn,
    pub token: String,
}

impl juniper::Context for Context {}

pub struct Query;

#[juniper::object(
// Here we specify the context type for this object.
Context = Context,
)]
impl Query {
    #[graphql(description = "Returns true if user exists in database and JWT not expiredT")]
    fn is_auth(context: &Context, jwt: Option<String>) -> FieldResult<bool> {
        let jwt = match jwt {
            Some(jwt) => jwt,
            None => return Ok(false)
        };

        let user_id = match crate::auth::get_user_id(&jwt) {
            Ok(user_id) => user_id,
            Err(e) => return Ok(false)
        };

        match User::is_in_db(&context.db_conn, user_id) {
            Ok(true) => Ok(true),
            _ => Ok(false),
        }
    }

    #[graphql(description = "Returns all web property stats for a users'")]
    fn private_mode_stats(context: &Context, property_id: juniper::ID) -> FieldResult<Option<Vec<PrivateCounts>>> {
        let user_id = match auth::get_user_id(&context.token) {
            Ok(user_id) => user_id,
            _ => return Err(juniper::FieldError::new("AUTHORIZATION_ERROR", graphql_value!({"error_description": "Not authorized: invalid token"})))
        };

        if Property::is_property_id_belong_to_user_id(&context.db_conn, &property_id, user_id) != Ok(true) {
            return Err(juniper::FieldError::new("AUTHORIZATION_ERROR", graphql_value!({"error_description": "Not found: no property id found for given user"})));
        }
        Ok(Event::get_private_stats(&context.db_conn, &property_id.deref()).ok())
    }

    #[graphql(description = "Returns a users' web property stats in the given date range. Date format: 'YYYY-MM-DD'")]
    fn private_mode_stats_by_date(context: &Context, property_id: juniper::ID, start_date: String, end_date: String) -> FieldResult<Option<Vec<PrivateCounts>>> {
        let user_id = match auth::get_user_id(&context.token) {
            Ok(user_id) => user_id,
            _ => return Err(juniper::FieldError::new("AUTHORIZATION_ERROR", graphql_value!({"error_description": "Not authorized: invalid token"})))
        };

        if Property::is_property_id_belong_to_user_id(&context.db_conn, &property_id, user_id) != Ok(true) {
            return Err(juniper::FieldError::new("AUTHORIZATION_ERROR", graphql_value!({"error_description": "Not found: no property id found for given user"})));
        }

        let start_date = match chrono::NaiveDate::parse_from_str(&start_date, "%Y-%m-%d") {
            Ok(date) => date,
            Err(_) => return Err(juniper::FieldError::new("INPUT_ERROR", graphql_value!({"error_description": "Date format is not ISO 8601 format, eg. YYYY-MM-DD"}))),
        };

        let end_date = match chrono::NaiveDate::parse_from_str(&end_date, "%Y-%m-%d") {
            Ok(date) => date,
            Err(_) => return Err(juniper::FieldError::new("INPUT_ERROR", graphql_value!({"error_description": "Date format is not ISO 8601 format, eg. YYYY-MM-DD"}))),
        };
        Ok(Event::get_private_stats_by_date(&context.db_conn, &property_id.deref(), &start_date, &end_date).ok())
    }

    #[graphql(description = "Returns a users' web properties")]
    fn get_properties(context: &Context) -> FieldResult<Vec<Property>> {
        let user_id = match auth::get_user_id(&context.token) {
            Ok(user_id) => user_id,
            _ => return Err(juniper::FieldError::new("AUTHORIZATION_ERROR", graphql_value!({"error_description": "Not authorized: invalid token"})))
        };

        match Property::get_properties(&context.db_conn, user_id) {
            Ok(properties) => Ok(properties),
            Err(e) => {
                eprintln!("Error querying for list of properties {}", e);
                Err(juniper::FieldError::new("UNKNOWN_ERROR", graphql_value!({"error_description": "Internal server error"})))
            }
        }
    }
}

pub struct Mutation;

#[juniper::object(Context = Context)]
impl Mutation {
    #[graphql(description = "Logs a user in, updating their token and returning a new JWT")]
    fn login(context: &Context, email: String, password: String) -> FieldResult<String> {
        match User::login(&context.db_conn, &email, &password) {
            Ok(user) => Ok(user.auth_bearer_token),
            Err(e) => Err(juniper::FieldError::new("AUTHENTICATION_ERROR",
                                                   graphql_value!({"error_description": "Not authorized: email or password is incorrect"}))),
        }
    }

    #[graphql(description = "Signs a user up, returning a JWT")]
    fn signup(context: &Context,
              first_name: String,
              last_name: String,
              email: String,
              password: String,
    ) -> FieldResult<String> {
        let user = User::insert(&context.db_conn,
                                &first_name,
                                &last_name,
                                &email,
                                &password);

        match user {
            Ok(user) => Ok(user.auth_bearer_token),
            Err(e) => {
                println!("Error inserting user: {}", e);
                Err(juniper::FieldError::new("DB_INSERT_ERROR", graphql_value!({"error_description": "Cannot save to db"})))
            }
        }
    }

    #[graphql(description = "Creates a new property under the user's account, returns the property")]
    fn create_property(context: &Context, website_name: String, website_url: String) -> FieldResult<Option<Property>> {
        let user = match User::from_jwt(&context.db_conn, &context.token) {
            Ok(user) => user,
            Err(_e) => return Err(juniper::FieldError::new("AUTHENTICATION_ERROR", graphql_value!({"error_description": "User not found"})))
        };

        let property_id = match Property::generate_property_id_for_user_id(&context.db_conn, user.id) {
            Ok(property_id) => property_id,
            Err(_e) => return Err(juniper::FieldError::new("SERVER_ERROR", graphql_value!({"error_description": "Could not generate property id"})))
        };

        match Property::insert(&context.db_conn,
                               &property_id,
                               &website_name,
                               &website_url,
                               user.id) {
            Ok(property) => Ok(Some(property)),
            Err(e) => {
                println!("Error inserting user: {}", e);
                Err(juniper::FieldError::new("DB_INSERT_ERROR", graphql_value!({"error_description": "Cannot save to db"})))
            }
        }
    }
}

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {})
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};

    #[test]
    fn test_signup_login() {
        let pool = crate::db::establish_connection_temp_db();
        crate::db::reset_db(&pool.get().expect("couldn't get db connection from pool"));

        let signup = r#"
        mutation {
          signup(
            firstName: "bob"
            lastName: "lob"
            email: "a@a.com"
            password: "pass"
          )
        }
    "#;
        let (res, errs) = juniper::execute(signup,
                                           None,
                                           &create_schema(),
                                           &Default::default(),
                                           &Context {
                                               db_conn: pool.get().expect("couldn't get db connection from pool"),
                                               token: "".to_string(),
                                           }).unwrap();

        let signup_jwt = res.as_object_value().unwrap()
            .get_field_value("signup").unwrap()
            .as_scalar_value::<String>().unwrap()
            .to_owned();
        let signup_jwt_clone = signup_jwt.clone();
        assert!(errs.is_empty());
        assert!(signup_jwt.len() > 0);
        assert!(auth::is_valid_token(signup_jwt.as_str()));
        assert_eq!(res, graphql_value!({"signup": signup_jwt}));

        let login = r#"
        query{
          login(email: "a@a.com", password: "pass")
        }
    "#;
        let (res, errs) = juniper::execute(login,
                                           None,
                                           &create_schema(),
                                           &Default::default(),
                                           &Context {
                                               db_conn: pool.get().expect("couldn't get db connection from pool"),
                                               token: "".to_string(),
                                           }).unwrap();

        let login_jwt = res.as_object_value().unwrap()
            .get_field_value("login").unwrap()
            .as_scalar_value::<String>().unwrap()
            .to_owned();
        let login_jwt_clone = login_jwt.clone();
        assert!(errs.is_empty());
        assert!(login_jwt.len() > 0);
        assert!(auth::is_valid_token(login_jwt.as_str()));
        assert_eq!(res, graphql_value!({"login": login_jwt}));
        assert_ne!(signup_jwt_clone, login_jwt_clone);
    }

    #[test]
    fn test_login_create_properties() {
        let pool = crate::db::establish_connection_temp_db();
        crate::db::reset_db(&pool.get().expect("couldn't get db connection from pool"));

        let signup_jwt_clone = {
            let signup = r#"
            mutation {
              signup(
                firstName: "bob"
                lastName: "lob"
                email: "a@a.com"
                password: "pass"
              )
            }
        "#;
            let (res, errs) = juniper::execute(signup,
                                               None,
                                               &create_schema(),
                                               &Default::default(),
                                               &Context {
                                                   db_conn: pool.get().expect("couldn't get db connection from pool"),
                                                   token: "".to_string(),
                                               }).unwrap();
            let signup_jwt = res.as_object_value().unwrap()
                .get_field_value("signup").unwrap()
                .as_scalar_value::<String>().unwrap()
                .to_owned();
            assert!(errs.is_empty());
            signup_jwt.clone()
        };

        let login_jwt_clone = {
            let login = r#"
            query{
              login(email: "a@a.com", password: "pass")
            }
        "#;
            let (res, errs) = juniper::execute(login,
                                               None,
                                               &create_schema(),
                                               &Default::default(),
                                               &Context {
                                                   db_conn: pool.get().expect("couldn't get db connection from pool"),
                                                   token: "".to_string(),
                                               }).unwrap();
            let login_jwt = res.as_object_value().unwrap()
                .get_field_value("login").unwrap()
                .as_scalar_value::<String>().unwrap()
                .to_owned();
            assert!(errs.is_empty());
            assert_ne!(signup_jwt_clone, login_jwt);
            login_jwt.clone()
        };

        {
            //login with login_jwt -> should work
            let create_property = r#"
            mutation{
              createProperty(websiteName:"foo", websiteUrl: "foo.com"){
                id
              }
            }
        "#;
            let (res, errs) = juniper::execute(create_property,
                                               None,
                                               &create_schema(),
                                               &Default::default(),
                                               &Context {
                                                   db_conn: pool.get().expect("couldn't get db connection from pool"),
                                                   token: login_jwt_clone,
                                               }).unwrap();

            let create_property = res.as_object_value().unwrap()
                .get_field_value("createProperty").unwrap()
                .as_object_value().unwrap()
                .get_field_value("id").unwrap()
                .as_scalar_value::<String>().unwrap()
                .to_owned();
            assert_eq!(create_property, "1-1");
            assert!(errs.is_empty());
        }

        {
            //login with signup_jwt -> should work since we dont invalidate
            let create_property = r#"
            mutation{
              createProperty(websiteName:"foo", websiteUrl: "foo.com"){
                id
              }
            }
        "#;
            let (res, errs) = juniper::execute(create_property,
                                               None,
                                               &create_schema(),
                                               &Default::default(),
                                               &Context {
                                                   db_conn: pool.get().expect("couldn't get db connection from pool"),
                                                   token: signup_jwt_clone,
                                               }).unwrap();

            let create_property = res.as_object_value().unwrap()
                .get_field_value("createProperty").unwrap()
                .as_object_value().unwrap()
                .get_field_value("id").unwrap()
                .as_scalar_value::<String>().unwrap()
                .to_owned();
            assert_eq!(create_property, "1-2");
            assert!(errs.is_empty());
        }

        {
            //should fail with invalid token
            let create_property = r#"
            mutation{
              createProperty(websiteName:"foo", websiteUrl: "foo.com"){
                id
              }
            }
        "#;
            let (_res, errs) = juniper::execute(create_property,
                                                None,
                                                &create_schema(),
                                                &Default::default(),
                                                &Context {
                                                    db_conn: pool.get().expect("couldn't get db connection from pool"),
                                                    token: "".to_string(),
                                                }).unwrap();

            assert!(!errs.is_empty());
        }
    }

    #[actix_rt::test]
    async fn test_create_events_and_fetch_events() {
        let pool = crate::db::establish_connection_temp_db();
        crate::db::reset_db(&pool.get().expect("couldn't get db connection from pool"));

        let signup_jwt_clone = {
            let signup = r#"
            mutation {
              signup(
                firstName: "bob"
                lastName: "lob"
                email: "a@a.com"
                password: "pass"
              )
            }
        "#;
            let (res, errs) = juniper::execute(signup,
                                               None,
                                               &create_schema(),
                                               &Default::default(),
                                               &Context {
                                                   db_conn: pool.get().expect("couldn't get db connection from pool"),
                                                   token: "".to_string(),
                                               }).unwrap();
            let signup_jwt = res.as_object_value().unwrap()
                .get_field_value("signup").unwrap()
                .as_scalar_value::<String>().unwrap()
                .to_owned();
            assert!(errs.is_empty());
            signup_jwt.clone()
        };

        let login_jwt_clone = {
            let login = r#"
            query{
              login(email: "a@a.com", password: "pass")
            }
        "#;
            let (res, errs) = juniper::execute(login,
                                               None,
                                               &create_schema(),
                                               &Default::default(),
                                               &Context {
                                                   db_conn: pool.get().expect("couldn't get db connection from pool"),
                                                   token: "".to_string(),
                                               }).unwrap();
            let login_jwt = res.as_object_value().unwrap()
                .get_field_value("login").unwrap()
                .as_scalar_value::<String>().unwrap()
                .to_owned();
            assert!(errs.is_empty());
            assert_ne!(signup_jwt_clone, login_jwt);
            login_jwt.clone()
        };

        {
            //login with login_jwt -> should work
            let create_property = r#"
            mutation{
              createProperty(websiteName:"foo", websiteUrl: "foo.com"){
                id
              }
            }
        "#;
            let (res, errs) = juniper::execute(create_property,
                                               None,
                                               &create_schema(),
                                               &Default::default(),
                                               &Context {
                                                   db_conn: pool.get().expect("couldn't get db connection from pool"),
                                                   token: login_jwt_clone.clone(),
                                               }).unwrap();

            let create_property = res.as_object_value().unwrap()
                .get_field_value("createProperty").unwrap()
                .as_object_value().unwrap()
                .get_field_value("id").unwrap()
                .as_scalar_value::<String>().unwrap()
                .to_owned();
            assert_eq!(create_property, "1-1");
            assert!(errs.is_empty());
        }

        {
            //add first event
            let data = crate::handler::NewEvent {
                property_id: "1-1".to_string(),
                url: "foo.com".to_string(),
                user_agent: "Firefox?".to_string(),
                fingerprint: "SOME_FINGERPRINT".to_string(),
                is_private: true,
            };
            let mut app = test::init_service(App::new()
                .data(pool.clone())
                .route("/event", web::post().to(crate::handler::event)))
                .await;
            let req = test::TestRequest::post()
                .uri("/event")
                // .header("authorization", format!("Bearer {}", signup_jwt_clone))
                .set_json(&data)
                .to_request();

            let resp = test::call_service(&mut app, req).await;
            assert!(resp.status().is_success());
        }

        {
            //add second event
            let data = crate::handler::NewEvent {
                property_id: "1-1".to_string(),
                url: "foo.com".to_string(),
                user_agent: "Firefox?".to_string(),
                fingerprint: "SOME_FINGERPRINT".to_string(),
                is_private: false,
            };
            let mut app = test::init_service(App::new()
                .data(pool.clone())
                .route("/event", web::post().to(crate::handler::event)))
                .await;
            let req = test::TestRequest::post()
                .uri("/event")
                // .header("authorization", format!("Bearer {}", signup_jwt_clone))
                .set_json(&data)
                .to_request();

            let resp = test::call_service(&mut app, req).await;
            assert!(resp.status().is_success());
        }

        {
            //add non-existent property event
            let data = crate::handler::NewEvent {
                property_id: "1-99999".to_string(),
                url: "foo.com".to_string(),
                user_agent: "Firefox?".to_string(),
                fingerprint: "SOME_FINGERPRINT".to_string(),
                is_private: false,
            };
            let mut app = test::init_service(App::new()
                .data(pool.clone())
                .route("/event", web::post().to(crate::handler::event)))
                .await;
            let req = test::TestRequest::post()
                .uri("/event")
                // .header("authorization", format!("Bearer {}", signup_jwt_clone))
                .set_json(&data)
                .to_request();

            let resp = test::call_service(&mut app, req).await;
            assert!(!resp.status().is_success());
        }

        {
            //get the recently inserted 2 events
            let private_mode_stats = r#"
                  query{
                          privateModeStats(propertyId: "1-1")
                        }
                  "#;
            let (res, errs) = juniper::execute(private_mode_stats,
                                               None,
                                               &create_schema(),
                                               &Default::default(),
                                               &Context {
                                                   db_conn: pool.get().expect("couldn't get db connection from pool"),
                                                   token: login_jwt_clone.clone(),
                                               }).unwrap();

            let private_mode_stats: Vec<f64> = res.as_object_value().unwrap()
                .get_field_value("privateModeStats").unwrap()
                .as_list_value().unwrap()
                .to_owned()
                .iter()
                .map(|e: &juniper::Value| e.as_scalar_value::<f64>().unwrap().to_owned())
                .collect();
            assert_eq!(private_mode_stats, vec![1.0, 0.0]);
            assert!(errs.is_empty());
        }
    }
}