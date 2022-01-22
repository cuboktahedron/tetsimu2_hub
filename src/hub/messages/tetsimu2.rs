pub mod analyze_pc;
pub mod header;
pub mod init_tutor;
pub mod notify_status;
pub mod term_tutor;

use crate::hub::messages::tetsimu2::analyze_pc::AnalyzePcMessageReq;
use crate::hub::messages::tetsimu2::init_tutor::InitTutorMessageReq;
use crate::hub::messages::tetsimu2::notify_status::NotifyStatusMessageReq;
use crate::hub::messages::tetsimu2::term_tutor::TermTutorMessageReq;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub enum Tetsimu2Message {
  AnalyzePc(AnalyzePcMessageReq),
  NotifyStatus(NotifyStatusMessageReq),
  InitTutor(InitTutorMessageReq),
  TermTutor(TermTutorMessageReq),
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Tetsimu2MessageHeader {
  pub message_id: String,
}
