use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct HubMessageHeader {
  pub version: String,
  pub message_id: String,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct HubMessageResHeader {
  pub version: String,
  pub message_id: String,
  pub request_message_id: String,
  pub result: i32,
}
