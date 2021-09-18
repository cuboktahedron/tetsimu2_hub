use crate::hub::messages::hub::header::HubMessageHeader;
use crate::hub::messages::hub::log::LogMessage;
use crate::hub::messages::hub::log::LogMessageBody;
use crate::hub::messages::hub::HubMessage;
use crate::hub::messages::tetsimu2::Tetsimu2Message;
use std::io::stdin;
use std::sync::mpsc::Sender;
use std::thread;
use uuid::Uuid;

pub struct Cli {
  t2_t: Sender<Tetsimu2Message>,
  hub_t: Sender<HubMessage>,
}

impl Cli {
  pub fn start(t2_t: Sender<Tetsimu2Message>, hub_t: Sender<HubMessage>) {
    thread::spawn(|| {
      let cli = Cli { t2_t, hub_t };
      cli.main_loop();
    });
  }

  fn main_loop(&self) {
    loop {
      print!("99: Exit\n");

      let answer = self.get_input();
      match answer.as_str() {
        "1" => self.log(),
        "99" => break,
        _ => (),
      }
    }

    println!("Bye");
  }

  fn log(&self) {
    let message = LogMessage {
      header: HubMessageHeader {
        message_id: Uuid::new_v4().to_string(),
      },
      body: LogMessageBody {
        message: String::from("test"),
      },
    };
    println!("message sent: {:?}", message);
    if let Err(e) = self.hub_t.send(HubMessage::LogMessage(message)) {
      eprint!("{:?}", e);
    }
  }

  fn get_input(&self) -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).ok();
    return String::from(input.trim());
  }
}
