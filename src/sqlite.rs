// https://github.com/rusqlite/rusqlite

// https://www.sqlite.org/loadext.html
use rusqlite::{params, Connection, Result};
use std::fs;

use crate::models::person::*;
use crate::models::system::*;

fn get_db() -> std::result::Result<Connection, rusqlite::Error> {
  let path = "./chegde-v.db";

  return Connection::open(&path);
}

pub fn test_sqlite() -> Result<()> {
  let conn = get_db()?;

  let me = Person {
    id: 0,
    name: "Steven".to_string(),
    data: None,
  };
  conn.execute(
    "INSERT INTO person (name, data) VALUES (?1, ?2)",
    params![me.name, me.data],
  )?;

  let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
  let person_iter = stmt.query_map([], |row| {
    Ok(Person {
      id: row.get(0)?,
      name: row.get(1)?,
      data: row.get(2)?,
    })
  })?;

  for person in person_iter {
    println!("Found person {:?}", person.unwrap());
  }
  Ok(())
}

pub fn insert(note: Note) -> Result<()> {
  let conn = get_db()?;

  conn.execute(
    "INSERT INTO note (id, title, content, user_id, type, create_time, object_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
    params![note.id, note.title, note.content, note.user_id, note.r#type, note.create_time, note.object_id],
  )?;
  return Ok(());
}

pub fn create_table() -> Result<()> {
  let conn = get_db()?;
  let sql_text = fs::read_to_string("./table.sql").unwrap();
  println!("{}", sql_text);

  conn.execute(sql_text.as_str(), [])?;

  return Ok(());
}
