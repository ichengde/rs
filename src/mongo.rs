extern crate dotenv;
use dotenv::dotenv;
use std::env;

use futures::stream::StreamExt;
use mongodb::{
  bson::{doc, Bson, Document},
  options::FindOptions,
};
use mongodb::{options::ClientOptions, Client};

use crate::models::system::*;

use crate::sqlite;

pub async fn mongo_test() -> () {
  dotenv().ok();

  let database_url = env::var("MONGODB_URL").expect("MONGODB_URL must be set");

  let client = Client::with_uri_str(&database_url).await.unwrap();

  // for db_name in client.list_database_names(None, None).await.unwrap() {
  //   println!("{}", db_name);
  // }

  let db = client.database("note");

  let note_entity = db.collection("noteEntity");

  let filter = doc! {};
  let find_options = FindOptions::builder().sort(doc! { "title": 1 }).build();
  let mut cursor = note_entity.find(filter, find_options).await.unwrap();

  // Iterate over the results of the cursor.

  while let Some(result) = cursor.next().await {
    match result {
      Ok(document) => {
        // println!("document: {:?}", document);

        let createTime = document.get("createTime").and_then(Bson::as_datetime);

        let title = document.get("title").and_then(Bson::as_str);

        let note = Note {
          id: None,
          object_id: document
            .get("_id")
            .and_then(Bson::as_object_id)
            .unwrap()
            .to_hex(),
          title: document
            .get("title")
            .and_then(Bson::as_str)
            .unwrap()
            .to_string(),
          content: document
            .get("content")
            .and_then(Bson::as_str)
            .unwrap()
            .to_string(),
          r#type: document.get("type").and_then(Bson::as_i32),
          create_time: document
            .get("createTime")
            .and_then(Bson::as_datetime)
            .unwrap()
            .timestamp(),
          user_id: document
            .get("userId")
            .and_then(Bson::as_str)
            .unwrap()
            .to_string(),
        };

        sqlite::insert(note).unwrap();
      }
      Err(e) => println!("{:?}", e),
    }
  }
}
