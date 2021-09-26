use crate::hub::messages::hub::header::HubMessageHeader;
use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct LogMessage {
  pub header: HubMessageHeader,
  pub body: LogMessageBody,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct LogMessageBody {
  pub message: String,
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
