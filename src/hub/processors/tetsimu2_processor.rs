use crate::hub::messages::tetsimu2::Tetsimu2Message;

pub trait Tetsimu2Processor {
  fn execute(&self, message: &Tetsimu2Message);
  fn before_execute(&self, message: &Tetsimu2Message) -> BeforeExecuteResult;
  fn halt(&self);
}

pub enum BeforeExecuteResult {
  Allow,
  Deny,
  Halt,
  Done,
}
