// use actix_service::Service;

use actix_web::{get, web, HttpRequest, Responder};
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

use crate::sqlite::query;

#[get("/")]
async fn home_controller() -> impl Responder {
    format!("hi")
}

#[get("/token")]
pub async fn token_controller() -> impl Responder {
    format!("Hello",)
}

#[derive(Serialize)]
struct Error {
    message: String,
}

#[get("/note/detail/{object_id}")]
pub async fn note_detail(req: HttpRequest) -> impl Responder {
    let object_id = req.match_info().get("object_id").unwrap();
    println!("{}", object_id);
    let detail = query(object_id).unwrap();

    return web::Json(detail);
}

use crate::diesel::prelude::*;
use crate::models::system::*;
use crate::postgres::*;

#[get("/get_website")]
pub async fn get_website() -> impl Responder {
    use crate::schema::website::dsl::*;
    let connection = establish_connection();
    let results = website
        .load::<Website>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());

    let res = ResponseType { data: results };

    return web::Json(res);
}
