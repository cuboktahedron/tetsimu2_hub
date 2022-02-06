use crate::hub::messages::hub::header::HubMessageResHeader;
use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum TermTutorMessageResResult {
  Succeeded = 0,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct TermTutorMessageRes {
  pub header: HubMessageResHeader,
  pub body: TermTutorMessageResBody,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct TermTutorMessageResBody {}
