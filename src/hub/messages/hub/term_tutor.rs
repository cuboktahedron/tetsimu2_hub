use crate::hub::messages::hub::header::HubMessageResHeader;
use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum TermTutorMessageResResult {
  Succeeded = 0,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct TermTutorMessageRes {
  pub header: HubMessageResHeader,
  pub body: TermTutorMessageResBody,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct TermTutorMessageResBody {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn serialize() {
    let actual = serde_json::to_string(&TermTutorMessageRes {
      header: HubMessageResHeader {
        message_id: String::from("abcd"),
        request_message_id: String::from("efgh"),
        result: 0,
      },
      body: TermTutorMessageResBody {},
    })
    .unwrap();

    #[rustfmt::skip]
    let expected = String::from("")
        + "{"
          + "\"header\":{"
            + "\"message_id\":\"abcd\","
            + "\"request_message_id\":\"efgh\","
            + "\"result\":0"
          + "},"
          + "\"body\":{}"
        + "}";

    assert_eq!(actual, expected);
  }
}
