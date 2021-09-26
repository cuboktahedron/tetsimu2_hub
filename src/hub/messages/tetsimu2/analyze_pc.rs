use crate::hub::messages::tetsimu2::header::Tetsimu2MessageHeader;
use serde::Deserialize;
use serde_big_array::big_array;

big_array! { BigArray; 300 }

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct AnalyzePcMessageReq {
  pub header: Tetsimu2MessageHeader,
  pub body: AnalyzePcMessageReqBody,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct AnalyzePcMessageReqBody {
  #[serde(with = "BigArray")]
  pub field: [u8; 300],
  pub nexts: String,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn deserialize() {
    let actual = serde_json::from_str::<AnalyzePcMessageReq>(
      r#"
    {
      "header": {
        "message_id": "abcd"
      },
      "body": {
        "field": [
          0,0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,0
        ],
        "nexts": "IJLOSTZ"
      }
    }"#,
    )
    .unwrap();

    let expected = AnalyzePcMessageReq {
      header: Tetsimu2MessageHeader {
        message_id: String::from("abcd"),
      },
      body: AnalyzePcMessageReqBody {
        field: [0; 300],
        nexts: String::from("IJLOSTZ"),
      },
    };

    assert_eq!(actual, expected);
  }
}
