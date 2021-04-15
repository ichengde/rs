use std::fs::read_to_string;
use std::fs::File;
use std::io::prelude::*;
use std::string::String;

pub fn write_token(id: &String) -> std::io::Result<()> {
  let foo = read_to_string("foo.txt");
  match foo {
    Ok(f) => {
      let mut buffer = String::new();
      buffer.push_str(id);
      buffer.push_str("\n");
      buffer.push_str(&f);
      let mut n1 = File::create("foo.txt")?;

      n1.write_all(buffer.as_bytes())?;
      Ok(())
    }
    Err(_err) => Ok(()),
  }
}
