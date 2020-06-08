use std::sync::Arc;
use std::borrow::Cow;
use rust_embed::RustEmbed;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web::body::Body;
use actix_web::http::header::Header;
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use crate::db::DbPool;
use crate::graphql;
use crate::models::{Event};

//from https://github.com/pyros2097/rust-embed/blob/master/examples/actix.rs
#[derive(RustEmbed)]
#[folder = "../client/build"]
struct Asset;

fn handle_embedded_file(path: &str) -> HttpResponse {
    match Asset::get(path) {
        Some(content) => {
            let body: Body = match content {
                Cow::Borrowed(bytes) => bytes.into(),
                Cow::Owned(bytes) => bytes.into(),
            };
            HttpResponse::Ok().content_type(mime_guess::from_path(path).first_or_octet_stream().as_ref()).body(body)
        }
        None => index(),
    }
}

pub fn index() -> HttpResponse {
    handle_embedded_file("index.html")
}

pub fn dist(req: HttpRequest) -> HttpResponse {
    let path = &req.path()["/".len()..]; // trim the preceding `/` in path
    handle_embedded_file(path)
}

pub fn graphiql() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(juniper::http::playground::playground_source("/graphql"))
}

pub async fn graphql(
    schema: web::Data<Arc<graphql::Schema>>,
    pool: web::Data<DbPool>,
    request: web::Json<juniper::http::GraphQLRequest>,
    raw_request: HttpRequest, //needed to extract the http authorization bearer token
) -> Result<HttpResponse, Error> {
    let token = match Authorization::<Bearer>::parse(&raw_request) {
        Ok(auth) => auth.into_scheme().token().to_string(),
        _ => "".to_string(), //no valid token
    };
    println!("user provided token: {}", token);
    let user = web::block(move || {
        let res = request.execute(&schema,
                                  &graphql::Context {
                                      db_conn: pool.get().expect("Couldn't get db connection from pool"),
                                      token,
                                  });
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
        .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(user))
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct NewEvent {
    pub property_id: String,
    pub url: String,
    pub user_agent: String,
    pub fingerprint: String,
    pub is_private: bool,
}

pub async fn event(pool: web::Data<DbPool>,
                   request: web::Json<NewEvent>,
                   raw_request: HttpRequest, //needed to extract the http authorization bearer token
) -> HttpResponse {
    let conn = pool.get().expect("Couldn't get db connection from pool");
    // let token = match Authorization::<Bearer>::parse(&raw_request) {
    //     Ok(auth) => auth.into_scheme().token().to_string(),
    //     Err(_) => return HttpResponse::Unauthorized().finish()
    // };
    //
    // let user_id = match crate::auth::get_user_id(&token) {
    //     Ok(user_id) => user_id,
    //     _ => return HttpResponse::Unauthorized().finish(),
    // };
    //
    // if Property::is_property_id_belong_to_user_id(&conn, &request.property_id, user_id) != Ok(true) {
    //     return HttpResponse::Unauthorized().finish();
    // }
    match Event::insert(&conn,
                        &request.url,
                        &raw_request.connection_info().host(),
                        &request.user_agent,
                        &request.fingerprint,
                        request.is_private,
                        &request.property_id) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("Event insert error {}", e);
            HttpResponse::Unauthorized().finish()
        }
    }
}

/*
curl --header "Content-Type: application/json" \
  --request POST \
  --data '{"username":"xyz","password":"xyz"}' \
  http://localhost:3000/api/login
 */
