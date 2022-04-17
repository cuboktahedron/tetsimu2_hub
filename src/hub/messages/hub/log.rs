use crate::hub::messages::hub::header::HubMessageHeader;
use crate::hub::messages::hub::VERSION;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct LogMessage {
  pub header: HubMessageHeader,
  pub body: LogMessageBody,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct LogMessageBody {
  pub message: String,
}

impl LogMessage {
  pub fn create(message: &str) -> LogMessage {
    LogMessage {
      header: HubMessageHeader {
        version: String::from(VERSION),
        message_id: Uuid::new_v4().to_string(),
      },
      body: LogMessageBody {
        message: String::from(message),
      },
    }
  }
}
