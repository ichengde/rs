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
struct Measurement {
    temperature: f32,
}

#[get("/note/detail/{object_id}")]
pub async fn note_detail(req: HttpRequest) -> impl Responder {
    let object_id = req.match_info().get("object_id").unwrap();

    return web::Json(query(object_id.to_string()).unwrap());
}
