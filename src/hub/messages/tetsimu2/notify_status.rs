use crate::hub::messages::tetsimu2::header::Tetsimu2MessageHeader;
use serde::Deserialize;
use serde_big_array::big_array;

big_array! { BigArray; 300 }

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct NotifyStatusMessageReq {
  pub header: Tetsimu2MessageHeader,
  pub body: NotifyStatusMessageReqBody,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct NotifyStatusMessageReqBody {
  #[serde(with = "BigArray")]
  pub field: [u8; 300],
  pub nexts: String,
  pub garbage_info: [u8; 13],
  pub can_hold: bool,
  pub hold_type: u8,
  pub ren: i8,
  pub is_btb: bool,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn deserialize() {
    let actual = serde_json::from_str::<NotifyStatusMessageReq>(
      r#"
    {
      "header": {
        "version": "1.0.0",
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
        "nexts": "IJLOSTZIJLOST",
        "garbage_info": [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        "hold_type": 0,
        "can_hold": true,
        "ren": 1,
        "is_btb": true
      }
    }"#,
    )
    .unwrap();

    let expected = NotifyStatusMessageReq {
      header: Tetsimu2MessageHeader {
        version: String::from("1.0.0"),
        message_id: String::from("abcd"),
      },
      body: NotifyStatusMessageReqBody {
        field: [0; 300],
        nexts: String::from("IJLOSTZIJLOST"),
        can_hold: true,
        hold_type: 0,
        garbage_info: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        ren: 1,
        is_btb: true,
      },
    };

    assert_eq!(actual, expected);
  }
}
