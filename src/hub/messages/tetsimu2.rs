pub mod analyze_pc;
pub mod header;

use crate::hub::messages::tetsimu2::analyze_pc::AnalyzePcMessage;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub enum Tetsimu2Message {
  AnalyzePc(AnalyzePcMessage),
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Tetsimu2MessageHeader {
  pub message_id: String,
}
