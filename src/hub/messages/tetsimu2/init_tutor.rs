use crate::hub::messages::tetsimu2::header::Tetsimu2MessageHeader;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct InitTutorMessageReq {
  pub header: Tetsimu2MessageHeader,
  pub body: InitTutorMessageReqBody,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct InitTutorMessageReqBody {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn deserialize() {
    let actual = serde_json::from_str::<InitTutorMessageReq>(
      r#"
    {
      "header": {
        "version": "1.0.0",
        "message_id": "abcd"
      },
      "body": {}
    }"#,
    )
    .unwrap();

    let expected = InitTutorMessageReq {
      header: Tetsimu2MessageHeader {
        version: String::from("1.0.0"),
        message_id: String::from("abcd"),
      },
      body: InitTutorMessageReqBody {},
    };

    assert_eq!(actual, expected);
  }
}
