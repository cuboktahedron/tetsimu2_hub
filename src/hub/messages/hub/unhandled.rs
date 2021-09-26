use crate::hub::messages::hub::header::HubMessageHeader;
use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct UnhandledMessage {
  pub header: HubMessageHeader,
  pub body: UnhandledMessageBody,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct UnhandledMessageBody {
  pub data: String,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn deserialize() {
    let actual = serde_json::to_string(&UnhandledMessage {
      header: HubMessageHeader {
        message_id: String::from("abcd"),
      },
      body: UnhandledMessageBody {
        data: String::from("Unhandled message"),
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
          + "\"data\":\"Unhandled message\""
        + "}"
      + "}";

    assert_eq!(actual, expected);
  }
}
