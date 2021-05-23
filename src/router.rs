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

#[derive(Deserialize, Serialize)]
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

// #[post("/login")]
// pub async fn login(info: web::Json<Info>) -> impl Responder {
//     let mut conn = pg_connect();
//     let row = &conn.await.query_one(
//         "SELECT * FROM public.user where username='$1' and password='$2'",
//         &[&info.username, &info.password],
//     );

//     let person = UserTable {
//         id: row.get(0),
//         username: row.get(1),
//         password: row.get(2),
//         object_id: row.get(3),
//     };
//     web::Json(person)
// }
