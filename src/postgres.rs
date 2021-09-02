use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use tokio_postgres::{Client, NoTls};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = get_database_url();
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub async fn pg_connect() -> Client {
    dotenv().ok();
    let mut database_url = get_database_url();
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

fn get_database_url() -> String {
    let uri = "postgres://$1:$2@$3:$4/$5";

    uri.replace("$1", env::var("PG.USER").unwrap().as_str())
        .replace("$2", env::var("PG.PASSWORD").unwrap().as_str())
        .replace("$3", env::var("PG.HOST").unwrap().as_str())
        .replace("$4", env::var("PG.PORT").unwrap().as_str())
        .replace("$5", env::var("PG.DBNAME").unwrap().as_str())
        .to_string()
}
