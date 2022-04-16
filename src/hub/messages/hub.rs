pub mod analyze_pc;
pub mod header;
pub mod init_tutor;
pub mod log;
pub mod steps;
pub mod term_tutor;
pub mod unhandled;
pub mod version;

use crate::hub::messages::hub::analyze_pc::AnalyzePcMessageRes;
use crate::hub::messages::hub::init_tutor::InitTutorMessageRes;
use crate::hub::messages::hub::log::LogMessage;
use crate::hub::messages::hub::steps::StepsMessage;
use crate::hub::messages::hub::term_tutor::TermTutorMessageRes;
use crate::hub::messages::hub::unhandled::UnhandledMessage;
use crate::hub::messages::hub::version::VersionMessage;
use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum HubMessage {
  AnalyzePc(AnalyzePcMessageRes),
  InitTutor(InitTutorMessageRes),
  Log(LogMessage),
  Steps(StepsMessage),
  TermTutor(TermTutorMessageRes),
  Unhandled(UnhandledMessage),
  Version(VersionMessage),
}
