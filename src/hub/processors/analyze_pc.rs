use crate::hub::messages::hub::analyze_pc::AnalyzePcMessageRes;
use crate::hub::messages::hub::analyze_pc::AnalyzePcMessageResBody;
use crate::hub::messages::hub::analyze_pc::AnalyzePcMessageResResult;
use crate::hub::messages::hub::header::HubMessageResHeader;
use crate::hub::messages::hub::log::LogMessage;
use crate::hub::messages::hub::HubMessage;
use crate::hub::messages::tetsimu2::analyze_pc::AnalyzePcMessageReq;
use crate::hub::messages::tetsimu2::Tetsimu2Message;
use crate::hub::processors::tetsimu2_processor::BeforeExecuteResult;
use crate::hub::processors::tetsimu2_processor::Tetsimu2Processor;
use crate::settings::Settings;
use crate::tetfu::core::Tetsimu2Content;
use crate::tetfu::tetfu_encoder::TetfuEncoder;
use crate::tetsimu2::core::FieldCellValue;
use crate::tetsimu2::core::MAX_FIELD_HEIGHT;
use crate::tetsimu2::core::MAX_FIELD_WIDTH;
use crate::tetsimu2::field::Field;
use anyhow::Context;
use anyhow::Result;
use core::convert::TryFrom;
use log::{debug, info, warn};
use num_traits::FromPrimitive;
use std::convert::TryInto;
use std::path::Path;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use uuid::Uuid;

const MAIN_JAR: &str = "sfinder.jar";

#[derive(Debug, PartialEq, Eq)]
enum DropType {
  SoftDrop = 0,
  HardDrop = 1,
  OneHundredEighty = 2,
  TSoftDrop = 3,
  AnyTSpin = 4,
  Tss = 5,
  Tsd = 6,
  Tst = 7,
}

impl TryFrom<u8> for DropType {
  type Error = String;

  fn try_from(n: u8) -> Result<Self, Self::Error> {
    let t = match n {
      0 => DropType::SoftDrop,
      1 => DropType::HardDrop,
      2 => DropType::OneHundredEighty,
      3 => DropType::TSoftDrop,
      4 => DropType::AnyTSpin,
      5 => DropType::Tss,
      6 => DropType::Tsd,
      7 => DropType::Tst,
      _ => return Err(format!("Cannot convert from '{}' to DropType", n)),
    };

    Ok(t)
  }
}

pub struct AnalyzePcProcesssor {
  out: ws::Sender,
  settings: Arc<Settings>,
  is_done: AtomicBool,
}

impl Tetsimu2Processor for AnalyzePcProcesssor {
  fn execute(&self, message: &Tetsimu2Message) {
    match message {
      Tetsimu2Message::AnalyzePc(m) => {
        self.execute_analyze_pc(&m);
      }
      _ => panic!("Passed message that cannnot be handled."),
    }
  }

  fn before_execute(&self, _message: &Tetsimu2Message) -> BeforeExecuteResult {
    if self.is_done.load(Ordering::Relaxed) {
      BeforeExecuteResult::Done
    } else {
      BeforeExecuteResult::Deny
    }
  }

  fn halt(&self) {
    info!("Halt.");
  }
}

impl AnalyzePcProcesssor {
  fn execute_analyze_pc(&self, message: &AnalyzePcMessageReq) {
    let request_result = self.execute_request(&message);
    self.execute_response(request_result, &message);
    // TODO: How to handle panic
    self.is_done.store(true, Ordering::Relaxed);
  }

  fn execute_request(&self, message: &AnalyzePcMessageReq) -> ExecuteRequestResult {
    self.log("Start analyze.");

    let settings = self.settings.clone();
    let sf_root = if let Some(x) = &settings.solution_finder.path {
      x
    } else {
      return ExecuteRequestResult::OtherError(String::from(
        "Solution finder settings is not set.",
      ));
    };

    if !Path::new(&sf_root).join(MAIN_JAR).exists() {
      return ExecuteRequestResult::OtherError(format!("Cannot find {}.", MAIN_JAR));
    }

    let data_vec = match message
      .body
      .field
      .iter()
      .map(|x| FromPrimitive::from_u8(*x).context("Could not '{}' to FieldCellValue."))
      .collect::<Result<Vec<FieldCellValue>>>()
    {
      Ok(x) => x,
      Err(e) => return ExecuteRequestResult::OtherError(format!("{:?}", e)),
    };

    let data: [FieldCellValue; 300] = match data_vec.try_into() {
      Ok(x) => x,
      Err(e) => return ExecuteRequestResult::OtherError(format!("{:?}", e)),
    };

    let field = Field { data };
    debug!("field:\n {:?}", field);

    let clear_line = if message.body.clear_line == 0 {
      self.decide_clear_line(&field)
    } else {
      message.body.clear_line as i32
    };
    debug!("clear_line: {}", clear_line);
    if clear_line == -1 {
      return ExecuteRequestResult::OtherError(String::from("Empty cell must be multiples of 4"));
    }

    let use_hold = if message.body.use_hold {
      "use"
    } else {
      "avoid"
    };

    let drop_type = match DropType::try_from(message.body.drop_type) {
      Ok(drop_type) => match drop_type {
        DropType::SoftDrop => "softdrop",
        DropType::HardDrop => "harddrop",
        DropType::Tss => "tss",
        DropType::Tsd => "tsd",
        DropType::Tst => "tst",
        _ => {
          return ExecuteRequestResult::OtherError(format!(
            "Unsupported drop type passed({})",
            drop_type as u8
          ))
        }
      },
      Err(e) => {
        return ExecuteRequestResult::OtherError(String::from(e));
      }
    };

    let tetfu_encoder = TetfuEncoder::new();
    let tetfu = tetfu_encoder.encode(&Tetsimu2Content {
      field,
      comment: String::from(""),
    });

    let output = Command::new("java")
      .arg("-jar")
      .arg(MAIN_JAR)
      .arg("path")
      .arg("--tetfu")
      .arg(tetfu)
      .arg("--patterns")
      .arg(&message.body.nexts)
      .arg("--clear-line")
      .arg(clear_line.to_string())
      .arg("--hold")
      .arg(use_hold)
      .arg("--drop")
      .arg(drop_type)
      .arg("--format")
      .arg("html")
      .current_dir(settings.solution_finder.path.clone().unwrap())
      .output();

    let output = match output {
      Ok(x) => x,
      Err(e) => {
        return ExecuteRequestResult::OtherError(e.to_string());
      }
    };

    debug!("status: {}", output.status);

    if output.status.success() {
      let stdout = String::from_utf8_lossy(&output.stdout);
      debug!("stdout:\n{}", &stdout);
      let found_paths = self.analyze_path_nums(&stdout);
      return ExecuteRequestResult::Succeeded(found_paths);
    } else {
      let err_message = String::from_utf8_lossy(&output.stderr);
      warn!("stderr:\n{}", err_message);

      let lines = err_message.split("\n").map(|s| s.trim());
      for line in lines {
        if line.starts_with("Message: ") {
          return ExecuteRequestResult::OtherError(String::from(&line["Message: ".len()..]));
        }
      }

      return ExecuteRequestResult::OtherError(String::from("Failed to analyze."));
    }
  }

  fn decide_clear_line(&self, field: &Field) -> i32 {
    let mut tmp_clear_line = 4;

    'outer: for y in (0..MAX_FIELD_HEIGHT).rev() {
      for x in 0..MAX_FIELD_WIDTH {
        if field.get_cell(x, y) != FieldCellValue::None {
          tmp_clear_line = y + 1;
          break 'outer;
        }
      }
    }

    let mut empty_cell_num = 0;
    for y in 0..tmp_clear_line {
      for x in 0..MAX_FIELD_WIDTH {
        if field.get_cell(x, y) == FieldCellValue::None {
          empty_cell_num += 1;
        }
      }
    }

    debug!("empty_cell_num: {}", empty_cell_num);

    if empty_cell_num % 2 == 1 {
      return -1;
    }

    if empty_cell_num % 4 == 0 {
      return tmp_clear_line;
    } else {
      return tmp_clear_line + 1;
    }
  }

  fn analyze_path_nums(&self, stdout: &str) -> String {
    let mut found_paths = vec![];
    let lines = stdout.split("\n").map(|s| s.trim());
    for line in lines {
      if line.starts_with("Found path") {
        found_paths.push(line);
      }
    }

    found_paths.join("\n")
  }

  fn execute_response(&self, request_result: ExecuteRequestResult, request: &AnalyzePcMessageReq) {
    let res_result = match request_result {
      ExecuteRequestResult::Succeeded(found_paths) => {
        self.execute_response_succeeced(request, found_paths)
      }
      ExecuteRequestResult::OtherError(message) => {
        self.execute_response_other_error(request, message)
      }
    };

    if res_result.is_err() {
      self
        .execute_response_other_error(request, String::from("Unexpected error occured."))
        .ok();
    }
  }

  fn execute_response_succeeced(
    &self,
    request: &AnalyzePcMessageReq,
    found_paths: String,
  ) -> Result<()> {
    let response = HubMessage::AnalyzePc(AnalyzePcMessageRes {
      header: HubMessageResHeader {
        message_id: Uuid::new_v4().to_string(),
        request_message_id: request.header.message_id.clone(),
        result: AnalyzePcMessageResResult::Succeeded as i32,
      },
      body: AnalyzePcMessageResBody {
        message: found_paths,
      },
    });

    let json = serde_json::to_string(&response)?;
    debug!("response:\n{}", json);
    self.out.send(json)?;

    Ok(())
  }

  fn execute_response_other_error(
    &self,
    request: &AnalyzePcMessageReq,
    message: String,
  ) -> Result<()> {
    let response = HubMessage::AnalyzePc(AnalyzePcMessageRes {
      header: HubMessageResHeader {
        message_id: Uuid::new_v4().to_string(),
        request_message_id: request.header.message_id.clone(),
        result: AnalyzePcMessageResResult::Succeeded as i32,
      },
      body: AnalyzePcMessageResBody {
        message: String::from(message),
      },
    });

    let json = serde_json::to_string(&response)?;
    debug!("response:\n{}", json);
    self.out.send(json)?;

    Ok(())
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

enum ExecuteRequestResult {
  Succeeded(String),
  OtherError(String),
}

pub fn execute(
  out: &ws::Sender,
  message: AnalyzePcMessageReq,
  settings: &Arc<Settings>,
) -> Arc<AnalyzePcProcesssor> {
  let processor = Arc::new(AnalyzePcProcesssor {
    out: out.clone(),
    settings: settings.clone(),
    is_done: AtomicBool::from(false),
  });

  let processor2 = Arc::clone(&processor);
  thread::spawn(move || {
    processor2.execute_analyze_pc(&message);
  });

  processor
}
