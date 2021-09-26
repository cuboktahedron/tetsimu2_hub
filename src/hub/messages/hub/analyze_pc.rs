use crate::hub::messages::hub::header::HubMessageResHeader;
use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct AnalyzePcMessageRes {
  pub header: HubMessageResHeader,
  pub body: AnalyzePcMessageResBody,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct AnalyzePcMessageResBody {
  pub message: String,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum AnalyzePcMessageResResult {
  Succeeded = 0,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn deserialize() {
    let actual = serde_json::to_string(&AnalyzePcMessageRes {
      header: HubMessageResHeader {
        message_id: String::from("abcd"),
        request_message_id: String::from("efgh"),
        result: 0,
      },
      body: AnalyzePcMessageResBody {
        message: String::from("succeeded."),
      },
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
          + "\"body\":{"
            + "\"message\":\"succeeded.\""
          + "}"
        + "}";

    assert_eq!(actual, expected);
  }
}
