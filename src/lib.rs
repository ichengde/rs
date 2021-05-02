#![feature(option_unwrap_none)]

#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod router;
pub mod sqlite;
pub mod models;
pub mod mongo;
pub mod schema;
pub mod postgres;