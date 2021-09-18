use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct HubMessageHeader {
  pub message_id: String,
}
