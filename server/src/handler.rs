use crate::db::DbPool;
use crate::graphql;
use crate::models::Event;
use actix_web::body::Body;
use actix_web::web::Json;
use actix_web::{web, Error, HttpRequest, HttpResponse, Result};
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use rust_embed::RustEmbed;
use std::borrow::Cow;

use crate::asyncgql::{BooksSchema, MutationRoot, QueryRoot, Storage, SubscriptionRoot};
use actix_web_actors::ws;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GQLRequest, GQLResponse, WSSubscription};

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
            HttpResponse::Ok()
                .content_type(mime_guess::from_path(path).first_or_octet_stream().as_ref())
                .body(body)
        }
        None => index(),
    }
}

pub fn index() -> HttpResponse {
    println!("index HttpResponse ");
    handle_embedded_file("index.html")
}

pub fn dist(req: HttpRequest) -> HttpResponse {
    let path = &req.path()["/".len()..]; // trim the preceding `/` in path
    handle_embedded_file(path)
}

// pub async fn graphql(
//     schema: web::Data<Arc<graphql::Schema>>,
//     pool: web::Data<DbPool>,
//     request: web::Json<juniper::http::GraphQLRequest>,
//     raw_request: HttpRequest, //needed to extract the http authorization bearer token
// ) -> Result<HttpResponse, Error> {
//     let token = match Authorization::<Bearer>::parse(&raw_request) {
//         Ok(auth) => auth.into_scheme().token().to_string(),
//         _ => "".to_string(), //no valid token
//     };
//     println!("user provided token: {}", token);
//     let user = web::block(move || {
//         let res = request.execute(&schema,
//                                   &graphql::Context {
//                                       db_conn: pool.get().expect("Couldn't get db connection from pool"),
//                                       token,
//                                   });
//         Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
//     })
//         .await?;
//     Ok(HttpResponse::Ok()
//         .content_type("application/json")
//         .body(user))
// }

pub(crate) async fn graphql(schema: web::Data<BooksSchema>, req: GQLRequest) -> GQLResponse {
    req.into_inner().execute(&schema).await.into()
}

pub(crate) async fn index_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
        )))
}

pub(crate) async fn index_ws(
    schema: web::Data<BooksSchema>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    ws::start_with_protocols(WSSubscription::new(&schema), &["graphql-ws"], &req, payload)
}
