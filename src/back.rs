// https://github.com/actix/actix-extras/tree/master/actix-web-httpauth
extern crate diesel;
extern crate dotenv;

use async_std::task;
// use actix_service::Service;
use actix_web::{
    error, get, http, http::StatusCode, middleware, post, web, App, HttpRequest, HttpResponse,
    HttpServer, Responder,
};
use tokio_postgres::{NoTls, Error};

use chegde_v::mongo::*;
use chegde_v::router::*;
use chegde_v::sqlite::*;

fn get_uri() -> &'static str {
    "127.0.0.1:8088"
}

#[tokio::main]
#[actix_web::main]
async fn start_server() -> std::io::Result<()> {
    println!("start server {}", get_uri());

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(chegde_v::middleware::cors())
            .service(token_controller)
            .service(home_controller)
            .service(note_detail)
            .service(get_website)
            // .service(login)
            .default_service(web::to(|| HttpResponse::Ok()))
    })
    .bind(get_uri())?
    .run()
    .await
}

fn main() {
    match start_server() {
        Err(e) => println!("{:?}", e),
        _ => (),
    }
    // task::block_on(mongo_test());

    // test_sqlite();
}
