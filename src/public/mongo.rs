extern crate dotenv;
use dotenv::dotenv;
use std::env;

use mongodb::{options::ClientOptions, Client};

pub async fn mongo_test() -> () {
  dotenv().ok();

  let database_url = env::var("MONGODB_URL").expect("MONGODB_URL must be set");
  // Parse a connection string into an options struct.
  let mut client_options = ClientOptions::parse(&database_url).await.unwrap();

  // Manually set an option.
  client_options.app_name = Some("My App".to_string());

  // Get a handle to the deployment.
  let client = Client::with_options(client_options).unwrap();

  // List the names of the databases in that deployment.
  for db_name in client.list_database_names(None, None).await.unwrap() {
    println!("{}", db_name);
  }
}
