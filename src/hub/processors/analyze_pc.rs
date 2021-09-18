use crate::hub::messages::tetsimu2::analyze_pc::AnalyzePcMessage;

pub fn execute(out: &ws::Sender, message: AnalyzePcMessage) {
  println!("message received: {:?}", message);
}
