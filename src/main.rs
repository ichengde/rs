// https://github.dev/actix/examples/tree/master/database_interactions/pg

#[macro_use]
extern crate diesel;

use dotenv::dotenv;
use tokio_postgres::NoTls;

use async_std::task;
// use actix_service::Service;
use crate::mongo::*;
use crate::router::*;
use crate::sqlite::*;
use actix_service::Service;
use actix_web::http::{header::CONTENT_TYPE, HeaderValue};
use actix_web::{
    error, get, http, http::StatusCode, middleware, post, web, App, HttpRequest, HttpResponse,
    HttpServer, Responder,
};
use router::*;

mod config;
mod cors;
mod db;
mod errors;
mod models;
mod mongo;
mod postgres;
mod redirect;
mod router;
mod schema;
mod sqlite;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .wrap(crate::cors::cors())
            .wrap_fn(|req, srv| {
                if req.path() == "/login" {
                    println!("call one");
                } else {
                }

                let headers = req.headers();
                let token = headers.get("token");

                match token {
                    Some(v) => {
                        println!("token {:?}", v);
                        let token_string = v.to_str().unwrap().to_string();

                        let claims = crate::redirect::jwt_decode(token_string);
                    }
                    None => {}
                }

                let fut = srv.call(req);

                fut
            })
            .service(token_controller)
            .service(home_controller)
            .service(note_detail)
            .service(get_website)
            .service(login)
    })
    .bind(config.server_addr.clone())?
    .run();
    println!("Server running at http://{}/", config.server_addr);

    server.await
}
