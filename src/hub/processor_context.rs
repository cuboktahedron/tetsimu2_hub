use crate::hub::hub_message_processor::HubMessageProcessor;
use crate::hub::tetsimu2_message_processor::Tetsimu2MessageProcessor;

pub struct ProcessorContext {
  pub hub_message_processor: HubMessageProcessor,
  pub tetsimu2_message_processor: Tetsimu2MessageProcessor,
}
