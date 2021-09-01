// use actix_service::Service;

use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::diesel::prelude::*;
use crate::models::system::*;
use crate::postgres::*;
use crate::sqlite::query;
use tokio_postgres::{Client, NoTls};

#[get("/")]
async fn home_controller() -> impl Responder {
    format!("hi")
}

#[get("/token")]
async fn token_controller() -> impl Responder {
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

#[get("/website")]
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

#[derive(Deserialize, Serialize, Debug)]
pub struct Info {
    username: String,
    password: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserTable {
    id: i32,
    username: String,
    password: String,
    object_id: String,
}

#[post("/login")]
pub async fn login(info: web::Json<Info>) -> impl Responder {
    let client = pg_connect();
    let row = client
        .await
        .query_one(
            "SELECT * FROM public.user where username='$1' and password='$2'",
            &[&info.username, &info.password],
        )
        .await;

    println!("{:?}", info);
    match row {
        Ok(r) => {
            let person = UserTable {
                id: r.get(0),
                username: r.get(1),
                password: r.get(2),
                object_id: r.get(3),
            };
            web::Json(person)
        }
        Err(err) => {
            println!("error {:?}", err);
            let person = UserTable {
                id: 12,
                username: "sad".to_string(),
                password: "sad".to_string(),
                object_id: "sad".to_string(),
            };
            web::Json(person)
        }
    }
}
