// use actix_service::Service;
use actix_web::{get, Responder};

#[get("/")]
async fn home_controller() -> impl Responder {
    format!("hi")
}

#[get("/token")]
pub async fn token_controller() -> impl Responder {
    format!("Hello",)
}

#[get("/note/detail")]
pub async fn note_detail() -> impl Responder {
    format!("Hello",)
}
