#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MsgErrorType {
  pub msg: Option<Vec<String>>,
  pub target: Option<String>,
  pub rowNum: Option<i32>,
  pub colNum: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Store {
  pub logType: String,
  pub logs: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JavascriptMsg {
  pub id: Option<String>,

  pub token: Option<String>,
  pub project: Option<String>,

  pub version: Option<String>,

  pub userAgent: Option<String>,
  pub location: Option<String>,
  pub store: Option<Vec<Store>>,
  pub error: Option<MsgErrorType>,
  pub createTime: Option<i32>,
}
