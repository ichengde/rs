// https://github.com/actix/actix-extras/tree/master/actix-web-httpauth
extern crate diesel;
extern crate dotenv;

use async_std::task;
use actix_cors::Cors;
// use actix_service::Service;
use actix_web::{
    error, get, http, http::StatusCode, post, web, App, HttpRequest, HttpResponse, HttpServer,
    Responder,
};

use chegde_v::mongo::*;
use chegde_v::router::*;
use chegde_v::sqlite::*;

fn get_uri() -> &'static str {
    "127.0.0.1:8088"
}

#[actix_web::main]
async fn start_server() -> std::io::Result<()> {
    println!("start server {}", get_uri());

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:52558")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .service(token_controller)
            .service(home_controller)
    })
    .bind(get_uri())?
    .run()
    .await
}

fn main() {
    // match start_server() {
    //     Err(e) => println!("{:?}", e),
    //     _ => (),
    // }
    task::block_on(mongo_test());


    // test_sqlite();
    println!("hello {}", "world!");
}
