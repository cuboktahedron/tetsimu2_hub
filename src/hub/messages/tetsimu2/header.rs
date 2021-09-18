use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Tetsimu2MessageHeader {
  pub message_id: String,
}
