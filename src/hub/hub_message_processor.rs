use crate::hub::messages::hub::HubMessage;
use std::sync::mpsc::Receiver;
use std::thread;

pub struct HubMessageProcessor {
  pub hub_r: Receiver<HubMessage>,
  pub out: ws::Sender,
}

impl HubMessageProcessor {
  pub fn start(hub_r: Receiver<HubMessage>, out: ws::Sender) {
    thread::spawn(move || {
      let processor = HubMessageProcessor { hub_r, out };

      processor.main_loop();
    });
  }

  fn main_loop(&self) {
    loop {
      match self.hub_r.recv() {
        Ok(v) => {
          let send_message = serde_json::to_string(&v).unwrap();
          self.out.send(send_message).ok();
        }
        Err(e) => {
          eprintln!("{:?}", e);
          break;
        }
      }
    }
  }
}
