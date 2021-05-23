use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub object_id: String,
}

#[derive(Debug, Serialize)]
pub struct Note {
    pub id: Option<i32>,
    pub title: String,
    pub content: String,
    pub user_id: String,
    pub r#type: Option<i32>,
    pub create_time: i64,
    pub object_id: String,
}

#[derive(Queryable, Serialize)]
pub struct Website {
    pub name: String,
    pub site: String,
}

#[derive(Serialize)]
pub struct ResponseType {
    pub data: Vec<Website>,
}
