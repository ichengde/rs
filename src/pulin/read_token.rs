use std::fs::File;
use std::io::prelude::*;
use std::string::String;

pub fn read_token() -> String {
  let f = File::open("foo.txt");
  let mut af = String::new();
  let mut buffer = [0; 8000];
  match f {
    Ok(mut thef) => {
      match thef.read(&mut buffer) {
        Ok(_) => {
          let ans = String::from_utf8(buffer.to_vec()).unwrap();
          af.push_str(&ans);
        }
        Err(_er) => {}
      };
      af
    }
    Err(_e) => af,
  }
}
