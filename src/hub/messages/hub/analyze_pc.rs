use crate::hub::messages::hub::header::HubMessageResHeader;
use serde::Serialize;
use serde_big_array::big_array;

big_array! { BigArray; 300 }

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct AnalyzePcMessageRes {
  pub header: HubMessageResHeader,
  pub body: AnalyzePcMessageResBody,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct AnalyzePcMessageResBody {
  pub message: String,
  pub minimal_items: Vec<AnalyzePcMessageResBodyItem>,
  pub unique_items: Vec<AnalyzePcMessageResBodyItem>,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct AnalyzePcMessageResBodyItem {
  pub title: String,
  pub detail: Vec<AnalyzePcMessageResBodyItemDetail>,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct AnalyzePcMessageResBodyItemDetail {
  pub settles: String,
  #[serde(with = "BigArray")]
  pub field: [u8; 300],
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum AnalyzePcMessageResResult {
  Succeeded = 0,
}
