use actix_web::{get, post, web, Either, HttpRequest, HttpResponse, Responder, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::diesel::prelude::*;
use crate::models::system::*;
use crate::postgres::*;
use crate::sqlite::query;

use crate::{db, errors::MyError, models::User};
use deadpool_postgres::{Client, Pool};

#[get("/")]
pub async fn home_controller() -> impl Responder {
    format!("hi")
}

#[get("/token")]
pub async fn token_controller() -> impl Responder {
    format!("Hello",)
}

#[derive(Serialize)]
pub struct Error {
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
pub async fn login(
    info: web::Json<Info>,
    db_pool: web::Data<Pool>,
) -> Either<web::Json<UserTable>, web::Json<Error>> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError).unwrap();
    println!("{:?}", info);

    let row = client
        .query_one(
            "SELECT * FROM public.user where username=$1 and password=$2",
            &[&info.username, &info.password],
        )
        .await;

    match row {
        Ok(r) => {
            let person = UserTable {
                id: r.get(0),
                username: r.get(1),
                password: r.get(2),
                object_id: r.get(3),
            };

            let too = crate::redirect::Claims {
                sub: person.object_id.clone(),
                exp: 600000000,
            };

            let res = crate::redirect::jwt_encode(too);
            println!("encode: {:?}", res);
            Either::Left(web::Json(person))
        }
        Err(err) => {
            println!("error {:?}", err);
            let person = Error {
                message: "adsf".to_string(),
            };
            Either::Right(web::Json(person))
        }
    }
}
