use crate::hub::messages::hub::header::HubMessageResHeader;
use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum InitTutorMessageResResult {
  Succeeded = 0,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct InitTutorMessageRes {
  pub header: HubMessageResHeader,
  pub body: InitTutorMessageResBody,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct InitTutorMessageResBody {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn serialize() {
    let actual = serde_json::to_string(&InitTutorMessageRes {
      header: HubMessageResHeader {
        message_id: String::from("abcd"),
        request_message_id: String::from("efgh"),
        result: 0,
      },
      body: InitTutorMessageResBody {},
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
