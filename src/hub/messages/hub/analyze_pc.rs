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
