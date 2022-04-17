use crate::hub::messages::hub::header::HubMessageHeader;
use crate::hub::messages::hub::VERSION;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct VersionMessage {
  pub header: HubMessageHeader,
  pub body: VersionMessageBody,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct VersionMessageBody {
  pub version: String,
}

impl VersionMessage {
  pub fn create(version: &str) -> VersionMessage {
    VersionMessage {
      header: HubMessageHeader {
        version: String::from(VERSION),
        message_id: Uuid::new_v4().to_string(),
      },
      body: VersionMessageBody {
        version: String::from(version),
      },
    }
  }
}
