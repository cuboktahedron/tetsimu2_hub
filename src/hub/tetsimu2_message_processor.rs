use crate::hub::messages::tetsimu2::Tetsimu2Message;
use crate::hub::processors;
use crate::settings::Settings;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::thread;

pub struct Tetsimu2MessageProcessor {
  pub t2_r: Receiver<Tetsimu2Message>,
  pub out: ws::Sender,
  pub settings: Arc<Settings>,
}

impl Tetsimu2MessageProcessor {
  pub fn start(t2_r: Receiver<Tetsimu2Message>, out: ws::Sender, settings: Arc<Settings>) {
    thread::spawn(|| {
      let processor = Tetsimu2MessageProcessor {
        t2_r,
        out,
        settings: settings,
      };

      processor.main_loop();
    });
  }

  fn main_loop(&self) {
    loop {
      match self.t2_r.recv() {
        Ok(v) => match v {
          Tetsimu2Message::AnalyzePc(m) => {
            processors::analyze_pc::execute(&self.out, m, &self.settings);
          }
        },
        Err(e) => {
          eprintln!("{:?}", e);
          break;
        }
      }
    }
  }
}
