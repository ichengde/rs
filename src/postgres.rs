use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use tokio_postgres::{Client, NoTls};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("connectURI").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub async fn pg_connect() -> Client {
    dotenv().ok();
    let mut database_url = env::var("connectURI").expect("DATABASE_URL must be set");
    let (client, connection) = tokio_postgres::connect(database_url.as_mut_str(), NoTls)
        .await
        .unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    return client;
}
