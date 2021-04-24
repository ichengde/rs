
#[derive(Debug)]
pub struct Person {
  pub id: i32,
  pub name: String,
  pub data: Option<Vec<u8>>,
}