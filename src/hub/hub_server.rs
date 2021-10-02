use crate::hub::messages::hub::header::HubMessageHeader;
use crate::hub::messages::hub::unhandled::UnhandledMessage;
use crate::hub::messages::hub::unhandled::UnhandledMessageBody;
use crate::hub::messages::hub::HubMessage;
use crate::hub::messages::tetsimu2::Tetsimu2Message;
use crate::hub::tetsimu2_message_processor::Tetsimu2MessageProcessor;
use crate::settings::Settings;
use anyhow::Context;
use anyhow::Result;
use log::info;
use serde_json::from_str;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use uuid::Uuid;
use ws::{listen, CloseCode, Handler, Handshake, Message};

pub struct HubServer {
  t2_t: Option<Sender<Tetsimu2Message>>,
  out: ws::Sender,
  settings: Arc<Settings>,
}

impl Handler for HubServer {
  fn on_open(&mut self, _handshake: Handshake) -> ws::Result<()> {
    info!("Connected[{}]", self.out.connection_id());

    let (t2_t, t2_r) = mpsc::channel::<Tetsimu2Message>();
    Tetsimu2MessageProcessor::start(t2_r, self.out.clone(), self.settings.clone());
    self.t2_t = Some(t2_t);

    Ok(())
  }

  fn on_message(&mut self, message: Message) -> ws::Result<()> {
    let received_message = format!("{}", message);
    let tetsimu2_message = from_str::<Tetsimu2Message>(&received_message);
    let ret = match tetsimu2_message {
      Ok(x) => self
        .t2_t
        .clone()
        .unwrap()
        .send(x)
        .context("Could not send received message to processor"),
      Err(_) => self
        .handle_unsupported_message()
        .context("Failed to handle unsupported message"),
    };

    match ret {
      Ok(_) => Ok(()),
      Err(e) => Err(ws::Error::new(ws::ErrorKind::Internal, format!("{:?}", e))),
    }
  }

  fn on_close(&mut self, code: CloseCode, reason: &str) {
    info!("Disconnected[{}] for ({:?}) {}", self.out.connection_id(), code, reason);
  }
}

impl HubServer {
  pub fn listen(settings: Arc<Settings>) -> anyhow::Result<()> {
    let endpoint = format!("{}:{}", settings.hub.host, settings.hub.port);
    let v = listen(endpoint, move |out| HubServer {
      out,
      t2_t: None,
      settings: settings.clone(),
    })?;
    Ok(v)
  }

  fn handle_unsupported_message(&self) -> Result<()> {
    let message = HubMessage::UnhandledMessage(UnhandledMessage {
      header: HubMessageHeader {
        message_id: String::from(Uuid::new_v4().to_string()),
      },
      body: UnhandledMessageBody {
        data: String::from("Unsupported message received."),
      },
    });

    let send_message = serde_json::to_string(&message)
      .context(format!("Failed to serialize message. ({:?})", message))?;

    self
      .out
      .send(send_message)
      .context("Failed to send 'UnhandledMessage'.")
  }
}
