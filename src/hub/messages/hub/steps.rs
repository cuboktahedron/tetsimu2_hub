use crate::hub::messages::hub::header::HubMessageHeader;
use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct StepsMessage {
  pub header: HubMessageHeader,
  pub body: StepsMessageBody,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct StepsMessageBody {
  pub request_message_id: String,
  pub steps: Vec<Step>,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct Step {
  pub r#type: u8,
  pub dir: u8,
  pub x: i8,
  pub y: i8,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn deserialize() {
    let actual = serde_json::to_string(&StepsMessage {
      header: HubMessageHeader {
        message_id: String::from("abcd"),
      },
      body: StepsMessageBody {
        request_message_id: String::from("123"),
        steps: vec![
          Step {
            r#type: 1,
            dir: 2,
            x: 3,
            y: 4,
          },
          Step {
            r#type: 2,
            dir: 3,
            x: 8,
            y: 0,
          },
        ],
      },
    })
    .unwrap();

    let expected = r#"{"header":{"message_id":"abcd"},"body":{"request_message_id":"123","steps":[{"type":1,"dir":2,"x":3,"y":4},{"type":2,"dir":3,"x":8,"y":0}]}}"#;

    assert_eq!(actual, expected);
  }
}
