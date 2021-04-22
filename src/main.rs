// https://github.com/actix/actix-extras/tree/master/actix-web-httpauth
extern crate diesel;
extern crate dotenv;

use actix_cors::Cors;
// use actix_service::Service;
use actix_web::{
    error, get, http, http::StatusCode, post, web, App, HttpRequest, HttpResponse, HttpServer,
    Responder,
};

use chegde_v::router::*;

#[get("/")]
async fn home_controller() -> impl Responder {
    format!("hi")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}

// pub fn create_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> SorryError {
//     use diesel_demo::schema::posts::dsl::*;

//     let connection = establish_connection();
//     let results = posts
//         .filter(published.eq(true))
//         .limit(5)
//         .load::<Post>(&connection)
//         .expect("Error loading posts");

//     println!("Displaying {} posts", results.len());
//     for post in results {
//         println!("{}", post.title);
//         println!("----------\n");
//         println!("{}", post.body);
//     }
// }
