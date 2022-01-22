use crate::hub::messages::hub::log::LogMessage;
use crate::hub::messages::hub::HubMessage;
use crate::hub::messages::tetsimu2::Tetsimu2Message;
use crate::hub::processors;
use crate::hub::processors::tetsimu2_processor::BeforeExecuteResult;
use crate::hub::processors::tetsimu2_processor::Tetsimu2Processor;
use crate::settings::Settings;
use log::{debug, error};
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::thread;

pub struct Tetsimu2MessageProcessor {
  pub t2_r: Receiver<Tetsimu2Message>,
  pub out: ws::Sender,
  pub settings: Arc<Settings>,
  processor: Option<Arc<dyn Tetsimu2Processor>>,
}

impl Tetsimu2MessageProcessor {
  pub fn start(t2_r: Receiver<Tetsimu2Message>, out: ws::Sender, settings: Arc<Settings>) {
    thread::spawn(|| {
      let mut processor = Tetsimu2MessageProcessor {
        t2_r,
        out,
        settings: settings,
        processor: None,
      };

      processor.main_loop();
    });
  }

  fn main_loop(&mut self) {
    println!("receive loop start");

    loop {
      let received_message = self.t2_r.try_recv();
      match received_message {
        Ok(message) => {
          if let Some(processor) = &self.processor {
            match processor.before_execute(&message) {
              BeforeExecuteResult::Deny => {
                self.log("Access denyed due to previous process is not done yet.");
                continue;
              }
              BeforeExecuteResult::Halt => {
                processor.halt();
                self.processor = None;
              }
              BeforeExecuteResult::Done => {
                self.processor = None;
              }
              _ => {}
            }
          }

          if let Some(processor) = &self.processor {
            processor.execute(&message)
          } else {
            match message {
              Tetsimu2Message::AnalyzePc(m) => {
                self.processor = Some(processors::analyze_pc::execute(
                  &self.out,
                  m,
                  &self.settings,
                ));
              }
              Tetsimu2Message::InitTutor(m) => {
                self.processor = Some(processors::tutor::execute(&self.out, m, &self.settings));
              }
              _ => {}
            }
          }
        }
        Err(e) => match e {
          std::sync::mpsc::TryRecvError::Disconnected => {
            error!("Connection disconnected.");
            if let Some(processor) = &self.processor {
              processor.halt();
            }
            break;
          }
          std::sync::mpsc::TryRecvError::Empty => {
            std::thread::sleep(std::time::Duration::from_millis(10));
          }
        },
      }
    }

    println!("mainloop end");
  }

  fn log(&self, message: &str) {
    let log = LogMessage::create(message);
    let message = HubMessage::Log(log);
    if let Ok(json) = serde_json::to_string(&message) {
      debug!("response:\n{}", json);
      self.out.send(json).ok();
    }
  }
}
