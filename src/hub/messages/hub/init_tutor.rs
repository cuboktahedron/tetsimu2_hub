use crate::hub::messages::hub::header::HubMessageResHeader;
use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum InitTutorMessageResResult {
  Succeeded = 0,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct InitTutorMessageRes {
  pub header: HubMessageResHeader,
  pub body: InitTutorMessageResBody,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct InitTutorMessageResBody {}
