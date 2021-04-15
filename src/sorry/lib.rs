#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod log;
pub mod models;
pub mod public;
pub mod pulin;
pub mod schema;

pub fn establish_connection() -> PgConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
use self::models::SorryError;

pub fn create_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> SorryError {
  use schema::sorry_errors;
  let new_sorry_errors = self::models::SorryErrorForm {
    title: title,
    body: Some(body),
  };

  diesel::insert_into(sorry_errors::table)
    .values(&new_sorry_errors)
    .get_result(conn)
    .expect("Error saving new post")
}
