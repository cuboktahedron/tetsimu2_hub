use crate::hub::messages::tetsimu2::header::Tetsimu2MessageHeader;
use serde::Deserialize;
use serde_big_array::big_array;

big_array! { BigArray; 300 }

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct AnalyzePcMessage {
  header: Tetsimu2MessageHeader,
  body: AnalyzePcMessageBody,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct AnalyzePcMessageBody {
  #[serde(with = "BigArray")]
  pub field: [u8; 300],
  pub nexts: String,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn deserialize() {
    let actual = serde_json::from_str::<AnalyzePcMessage>(
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

    let expected = AnalyzePcMessage {
      header: Tetsimu2MessageHeader {
        message_id: String::from("abcd"),
      },
      body: AnalyzePcMessageBody {
        field: [0; 300],
        nexts: String::from("IJLOSTZ"),
      },
    };

    assert_eq!(actual, expected);
  }
}
