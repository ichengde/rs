// use actix_service::Service;
use actix_web::{get, Responder};

#[get("/token")]
pub async fn token_controller() -> impl Responder {
    format!("Hello",)
}
