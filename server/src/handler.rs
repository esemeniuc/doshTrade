use std::borrow::Cow;

use actix_web::body::Body;
use actix_web::{web, HttpRequest, HttpResponse, Result};
use actix_web_actors::ws;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::Schema;
use async_graphql_actix_web::{Request, Response, WSSubscription};
use rust_embed::RustEmbed;

use crate::asyncgql::BooksSchema;

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

pub(crate) async fn graphql(schema: web::Data<BooksSchema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
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
    ws::start_with_protocols(
        WSSubscription::new(Schema::clone(&*schema)),
        &["graphql-ws"],
        &req,
        payload,
    )
}
