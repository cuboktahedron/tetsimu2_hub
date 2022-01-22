use crate::hub::messages::tetsimu2::header::Tetsimu2MessageHeader;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct TermTutorMessageReq {
  pub header: Tetsimu2MessageHeader,
  pub body: TermTutorMessageReqBody,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct TermTutorMessageReqBody {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn deserialize() {
    let actual = serde_json::from_str::<TermTutorMessageReq>(
      r#"
    {
      "header": {
        "message_id": "abcd"
      },
      "body": {}
    }"#,
    )
    .unwrap();

    let expected = TermTutorMessageReq {
      header: Tetsimu2MessageHeader {
        message_id: String::from("abcd"),
      },
      body: TermTutorMessageReqBody {},
    };

    assert_eq!(actual, expected);
  }
}
