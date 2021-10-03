use crate::hub::messages::hub::header::HubMessageHeader;
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
        message_id: Uuid::new_v4().to_string(),
      },
      body: LogMessageBody {
        message: String::from(message),
      },
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn deserialize() {
    let actual = serde_json::to_string(&LogMessage {
      header: HubMessageHeader {
        message_id: String::from("abcd"),
      },
      body: LogMessageBody {
        message: String::from("log message"),
      },
    })
    .unwrap();

    #[rustfmt::skip]
    let expected = String::from("")
      + "{"
        + "\"header\":{"
          + "\"message_id\":\"abcd\""
        + "},"
        + "\"body\":{"
          + "\"message\":\"log message\""
        + "}"
      + "}";

    assert_eq!(actual, expected);
  }
}
