use crate::hub::messages::hub::header::HubMessageHeader;
use crate::hub::messages::hub::header::HubMessageResHeader;
use crate::hub::messages::hub::init_tutor::InitTutorMessageRes;
use crate::hub::messages::hub::init_tutor::InitTutorMessageResBody;
use crate::hub::messages::hub::init_tutor::InitTutorMessageResResult;
use crate::hub::messages::hub::steps::Step;
use crate::hub::messages::hub::steps::StepsMessage;
use crate::hub::messages::hub::steps::StepsMessageBody;
use crate::hub::messages::hub::term_tutor::TermTutorMessageRes;
use crate::hub::messages::hub::term_tutor::TermTutorMessageResBody;
use crate::hub::messages::hub::term_tutor::TermTutorMessageResResult;
use crate::hub::messages::hub::HubMessage;
use crate::hub::messages::tetsimu2::init_tutor::InitTutorMessageReq;
use crate::hub::messages::tetsimu2::notify_status::NotifyStatusMessageReq;
use crate::hub::messages::tetsimu2::term_tutor::TermTutorMessageReq;
use crate::hub::messages::tetsimu2::Tetsimu2Message;
use crate::hub::processors::tetsimu2_processor::BeforeExecuteResult;
use crate::hub::processors::tetsimu2_processor::Tetsimu2Processor;
use crate::settings::Settings;
use crate::tetsimu2::core::FieldCellValue;
use crate::tetsimu2::core::Tetromino;
use crate::tetsimu2::core::MAX_FIELD_WIDTH;
use cold_clear;
use core::sync::atomic::{AtomicBool, Ordering};
use enumset::EnumSet;
use libtetris::*;
use log::{debug, error, info};
use num_traits::FromPrimitive;
use std::convert::TryFrom;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use std::time::Instant;
use uuid::Uuid;

impl From<Tetromino> for Piece {
  fn from(r#type: Tetromino) -> Self {
    match r#type {
      Tetromino::I => Piece::I,
      Tetromino::J => Piece::J,
      Tetromino::L => Piece::L,
      Tetromino::O => Piece::O,
      Tetromino::S => Piece::S,
      Tetromino::T => Piece::T,
      Tetromino::Z => Piece::Z,
    }
  }
}

impl From<Piece> for Tetromino {
  fn from(piece: Piece) -> Self {
    match piece {
      Piece::I => Tetromino::I,
      Piece::J => Tetromino::J,
      Piece::L => Tetromino::L,
      Piece::O => Tetromino::O,
      Piece::S => Tetromino::S,
      Piece::T => Tetromino::T,
      Piece::Z => Tetromino::Z,
    }
  }
}

impl From<crate::tetsimu2::core::Direction> for RotationState {
  fn from(dir: crate::tetsimu2::core::Direction) -> Self {
    match dir {
      crate::tetsimu2::core::Direction::Up => RotationState::North,
      crate::tetsimu2::core::Direction::Left => RotationState::West,
      crate::tetsimu2::core::Direction::Down => RotationState::South,
      crate::tetsimu2::core::Direction::Right => RotationState::East,
    }
  }
}

impl From<RotationState> for crate::tetsimu2::core::Direction {
  fn from(state: RotationState) -> Self {
    match state {
      RotationState::North => crate::tetsimu2::core::Direction::Up,
      RotationState::West => crate::tetsimu2::core::Direction::Left,
      RotationState::South => crate::tetsimu2::core::Direction::Down,
      RotationState::East => crate::tetsimu2::core::Direction::Right,
    }
  }
}

pub struct TutorProcessor {
  out: ws::Sender,
  #[allow(dead_code)]
  settings: Arc<Settings>,
  status: Mutex<Tetsimu2Status>,
  is_done: AtomicBool,
  tutor_if: Mutex<cold_clear::Interface>,
}

struct Tetsimu2Status {
  status_id: String,
  prev_steps: Vec<Step>,
  last_sent_time: Instant,
}

impl Default for Tetsimu2Status {
  fn default() -> Self {
    Tetsimu2Status {
      status_id: String::default(),
      prev_steps: Vec::default(),
      last_sent_time: Instant::now(),
    }
  }
}

impl Tetsimu2Processor for TutorProcessor {
  fn execute(&self, message: &Tetsimu2Message) {
    match message {
      Tetsimu2Message::InitTutor(m) => {
        self.initialize(&m);
      }
      Tetsimu2Message::TermTutor(m) => {
        self.terminate(&m);
      }
      Tetsimu2Message::NotifyStatus(m) => {
        self.update_current_stetus(&m);
      }
      _ => panic!("Passed message that cannnot be handled."),
    }
  }

  fn before_execute(&self, message: &Tetsimu2Message) -> BeforeExecuteResult {
    if self.is_done.load(Ordering::Relaxed) {
      BeforeExecuteResult::Done
    } else {
      match message {
        Tetsimu2Message::InitTutor(_) => BeforeExecuteResult::Allow,
        Tetsimu2Message::TermTutor(_) => BeforeExecuteResult::Allow,
        Tetsimu2Message::NotifyStatus(_) => BeforeExecuteResult::Allow,
        _ => BeforeExecuteResult::Deny,
      }
    }
  }

  fn halt(&self) {
    info!("Halted.");
    self.is_done.store(true, Ordering::Relaxed);
  }
}

impl TutorProcessor {
  fn main_loop(&self) {
    info!("TutorProcessor is ready.");

    while !self.is_done.load(Ordering::Relaxed) {
      std::thread::sleep(Duration::from_millis(100));

      let mut status = self.status.lock().unwrap();
      let request_message_id = status.status_id.clone();

      let tutor_if = self.tutor_if.lock().unwrap();

      tutor_if.suggest_next_move(0);
      let poll_result = tutor_if.poll_next_move();
      if let Err(_) = poll_result {
        continue;
      }
      let (_, info) = poll_result.ok().unwrap();

      let plan = match info {
        cold_clear::Info::Normal(info) => info.plan,
        cold_clear::Info::PcLoop(info) => info.plan,
        _ => {
          continue;
        }
      };

      let steps: Vec<_> = plan
        .into_iter()
        .map(|(falling_piece, _)| {
          let r#type = Tetromino::from(falling_piece.kind.0);
          let dir = crate::tetsimu2::core::Direction::from(falling_piece.kind.1);
          let mut x = falling_piece.x;
          let mut y = falling_piece.y;
          if falling_piece.kind.0 == Piece::I {
            match falling_piece.kind.1 {
              RotationState::West => {
                y += 1;
              }
              RotationState::South => {
                y -= 1;
                x += 1;
              }
              RotationState::East => {
                x -= 1;
              }
              _ => {}
            }
          }

          Step {
            r#type: r#type as u8,
            dir: dir as u8,
            x: x as i8,
            y: y as i8,
          }
        })
        .collect();

      if steps.is_empty() {
        status.prev_steps = steps;
        continue;
      }

      if status.prev_steps.is_empty() || steps[0] != status.prev_steps[0] {
        debug!("Steps changed.");

        status.prev_steps = steps;
        continue;
      }

      if status.prev_steps == steps && status.last_sent_time.elapsed().as_millis() < 1000 {
        continue;
      }

      status.prev_steps = steps.clone();

      let steps = HubMessage::Steps(StepsMessage {
        header: HubMessageHeader {
          message_id: Uuid::new_v4().to_string(),
        },
        body: StepsMessageBody {
          request_message_id: request_message_id,
          steps,
        },
      });

      let result = serde_json::to_string(&steps);
      if let Ok(json) = result {
        debug!("response:\n{}", json);
        if let Err(e) = self.out.send(json) {
          error!("{}", e);
        }
      }

      status.last_sent_time = Instant::now();
    }

    self.is_done.store(true, Ordering::Relaxed);
  }

  fn initialize(&self, message: &InitTutorMessageReq) {
    info!("Initializing.");

    let response = HubMessage::InitTutor(InitTutorMessageRes {
      header: HubMessageResHeader {
        message_id: Uuid::new_v4().to_string(),
        request_message_id: message.header.message_id.clone(),
        result: InitTutorMessageResResult::Succeeded as i32,
      },
      body: InitTutorMessageResBody {},
    });

    let json = serde_json::to_string(&response).unwrap();
    debug!("response:\n{}", json);
    self.out.send(json).ok();

    info!("Initialize done.");
  }

  fn terminate(&self, message: &TermTutorMessageReq) {
    info!("Terminating.");

    let response = HubMessage::TermTutor(TermTutorMessageRes {
      header: HubMessageResHeader {
        message_id: Uuid::new_v4().to_string(),
        request_message_id: message.header.message_id.clone(),
        result: TermTutorMessageResResult::Succeeded as i32,
      },
      body: TermTutorMessageResBody {},
    });

    let json = serde_json::to_string(&response).unwrap();
    debug!("response:\n{}", json);
    self.out.send(json).ok();
    self.is_done.store(true, Ordering::Relaxed);

    info!("Terminate done.");
  }

  fn update_current_stetus(&self, message: &NotifyStatusMessageReq) {
    debug!("Update current status. {:?}", message);

    let mut status = self.status.lock().unwrap();
    status.status_id = message.header.message_id.clone();

    if !message.body.can_hold {
      // This is because cold clear is not supported hold only movement
      return;
    }

    status.prev_steps = vec![];

    let mut field = [[false; 10]; 40];
    for cell in 0..message.body.field.len() {
      let x = cell % MAX_FIELD_WIDTH as usize;
      let y = cell / MAX_FIELD_WIDTH as usize;
      field[y][x] = message.body.field[cell as usize] != FieldCellValue::None as u8;
    }

    let hold_type: Option<Tetromino> = FromPrimitive::from_u8(message.body.hold_type);
    let hold = match hold_type {
      Some(t) => Some(Piece::from(t)),
      _ => None,
    };

    let combo = (message.body.ren + 1) as u32;
    let is_btb = message.body.is_btb;
    let board = Board::new_with_state(field, EnumSet::all(), hold, is_btb, combo).into();
    let tutor_if = cold_clear::Interface::launch(
      board,
      cold_clear::Options {
        speculate: false,
        ..Default::default()
      },
      cold_clear::evaluation::Standard::default(),
      None,
    );

    let nexts: Vec<_> = message
      .body
      .nexts
      .chars()
      .map(|c| match Tetromino::try_from(c) {
        Ok(r#type) => Piece::from(r#type),
        _ => panic!("Cannot convert '{}' into Piece", c),
      })
      .collect();

    for next in nexts {
      tutor_if.add_next_piece(next);
    }
    tutor_if.suggest_next_move(0);

    let mut prc_tutor_if = self.tutor_if.lock().unwrap();
    *prc_tutor_if = tutor_if;
  }
}

pub fn execute(
  out: &ws::Sender,
  message: InitTutorMessageReq,
  settings: &Arc<Settings>,
) -> Arc<TutorProcessor> {
  let processor = Arc::new(TutorProcessor {
    out: out.clone(),
    settings: settings.clone(),
    status: Mutex::new(Tetsimu2Status::default()),
    is_done: AtomicBool::from(false),
    tutor_if: Mutex::new(cold_clear::Interface::launch(
      Board::new(),
      cold_clear::Options {
        speculate: false,
        ..Default::default()
      },
      cold_clear::evaluation::Standard::default(),
      None,
    )),
  });

  let processor2 = Arc::clone(&processor);
  processor2.initialize(&message);

  thread::spawn(move || {
    processor2.main_loop();
  });

  processor
}
