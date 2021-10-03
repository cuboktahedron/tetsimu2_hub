pub mod analyze_pc;
pub mod header;
pub mod log;
pub mod unhandled;

use crate::hub::messages::hub::analyze_pc::AnalyzePcMessageRes;
use crate::hub::messages::hub::log::LogMessage;
use crate::hub::messages::hub::unhandled::UnhandledMessage;
use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum HubMessage {
  AnalyzePc(AnalyzePcMessageRes),
  Log(LogMessage),
  Unhandled(UnhandledMessage),
}
