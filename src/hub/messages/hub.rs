pub mod header;
pub mod log;
pub mod unhandled;

use crate::hub::messages::hub::log::LogMessage;
use crate::hub::messages::hub::unhandled::UnhandledMessage;
use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum HubMessage {
  LogMessage(LogMessage),
  UnhandledMessage(UnhandledMessage),
}
