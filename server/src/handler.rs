use std::borrow::Cow;
use rust_embed::RustEmbed;
use actix_web::{web, Error, HttpRequest, HttpResponse, Result};
use actix_web::web::Json;
use actix_web::body::Body;
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use crate::db::DbPool;
use crate::graphql;
use crate::models::{Event};
use web_push::*;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GQLRequest, GQLResponse, WSSubscription};
use actix_web_actors::ws;
use crate::asyncgql::{BooksSchema, MutationRoot, QueryRoot, Storage, SubscriptionRoot};

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
	println!("index HttpResponse ");
	handle_embedded_file("index.html")
}

fn generate_vapid_signature() -> VapidSignature {
	let subscription_info = SubscriptionInfo::new(
		"https://fcm.googleapis.com/fcm/send/fNbXUwtzYcU:APA91bFeBMjyuEy66oeyxtAJDMMtbt6uwyjNXZVNbsGmN2EN5twc9PtHlMNU-5NYVO0qsiGCwtT4Q4G-qU3y77aVEKZ_fEI1SeII3H_rXH5PWExLnXjFM-z9L_LtjYMapsLtveWq6BG0",
		"BNtxN65MgpFzq5VU_fnDe0PDZ6aI9LJHRLkL3Kh66fEW954d8a1xnaeJBzwmAHWt9ldD6V2ajW0GvISVxc4H-i8",
		"h3jhNyqaiptmEATj_5nQrQ",
	);

	let file = std::fs::File::open("private.pem").unwrap();

	let mut sig_builder = VapidSignatureBuilder::from_pem(file, &subscription_info).unwrap();
	sig_builder.add_claim("sub", "mailto:test@example.com");
	sig_builder.add_claim("foo", "bar");
	sig_builder.add_claim("omg", 123);

	return sig_builder.build().unwrap();
}

fn generate_push_message() -> Result<WebPushMessage, web_push::WebPushError> {
	let subscription_info = SubscriptionInfo::new(
		"https://fcm.googleapis.com/fcm/send/fNbXUwtzYcU:APA91bFeBMjyuEy66oeyxtAJDMMtbt6uwyjNXZVNbsGmN2EN5twc9PtHlMNU-5NYVO0qsiGCwtT4Q4G-qU3y77aVEKZ_fEI1SeII3H_rXH5PWExLnXjFM-z9L_LtjYMapsLtveWq6BG0",
		"BNtxN65MgpFzq5VU_fnDe0PDZ6aI9LJHRLkL3Kh66fEW954d8a1xnaeJBzwmAHWt9ldD6V2ajW0GvISVxc4H-i8",
		"h3jhNyqaiptmEATj_5nQrQ",
	);


	let mut builder = WebPushMessageBuilder::new(&subscription_info)?;
	let content = "Encrypted payload to be sent in the notification".as_bytes();
	builder.set_vapid_signature(generate_vapid_signature());
	builder.set_payload(ContentEncoding::AesGcm, content);
	return Ok(builder.build()?);
}

pub async fn send_it() {
	let client = WebPushClient::new();
	let message = generate_push_message().expect("failed to generate push message");
	let response = client.send(message).await;
	response
		.map_err(|e| println!("got error in sendit(), {} ", e))
		.map(|result| println!("Got response: {:?}", result));
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
