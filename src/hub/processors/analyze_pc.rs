use crate::hub::messages::hub::log::LogMessage;
use crate::hub::messages::hub::HubMessage;
use crate::tetsimu2::core::MAX_FIELD_HEIGHT;
use crate::tetsimu2::core::MAX_FIELD_WIDTH;
use log::{debug, warn};

use crate::hub::messages::hub::analyze_pc::AnalyzePcMessageRes;
use crate::hub::messages::hub::analyze_pc::AnalyzePcMessageResBody;
use crate::hub::messages::hub::analyze_pc::AnalyzePcMessageResResult;
use crate::hub::messages::hub::header::HubMessageResHeader;
use crate::hub::messages::tetsimu2::analyze_pc::AnalyzePcMessageReq;
use crate::settings::Settings;
use crate::settings::SolutionFinderSettings;
use crate::tetfu::core::Tetsimu2Content;
use crate::tetfu::tetfu_encoder::TetfuEncoder;
use crate::tetsimu2::core::FieldCellValue;
use crate::tetsimu2::field::Field;
use anyhow::Context;
use anyhow::Result;
use num_traits::FromPrimitive;
use std::convert::TryInto;
use std::path::Path;
use std::process::Command;
use std::sync::Arc;
use uuid::Uuid;

const MAIN_JAR: &str = "sfinder.jar";

enum ExecuteRequestResult {
  Succeeded(String),
  OtherError(String),
}

pub fn execute(out: &ws::Sender, message: AnalyzePcMessageReq, settings: &Arc<Settings>) {
  let request_result = execute_request(out, &message, &settings.solution_finder);
  execute_response(out, request_result, &message);
}

fn execute_request(
  out: &ws::Sender,
  message: &AnalyzePcMessageReq,
  settings: &SolutionFinderSettings,
) -> ExecuteRequestResult {
  log(out, "Start analyze");

  let sf_root = if let Some(x) = &settings.path {
    x
  } else {
    return ExecuteRequestResult::OtherError(String::from("Solution finder settings is not set."));
  };

  if !Path::new(&sf_root).join(MAIN_JAR).exists() {
    return ExecuteRequestResult::OtherError(format!("Cannot find {}.", MAIN_JAR));
  }

  let tetfu_encoder = TetfuEncoder::new();

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

  let clear_line = decide_clear_line(&field);
  debug!("clear_line: {}", clear_line);
  if clear_line == -1 {
    return ExecuteRequestResult::OtherError(String::from("Empty cell must be multiples of 4"));
  }

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
    .arg("--format")
    .arg("html")
    .current_dir(settings.path.clone().unwrap())
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
    let found_paths = analyze_path_nums(&stdout);
    return ExecuteRequestResult::Succeeded(found_paths);
  } else {
    let err_message = String::from_utf8_lossy(&output.stderr);
    warn!("stderr:\n{}", err_message);

    let lines = err_message.split("\n").map(|s| s.trim());
    for line in lines {
      println!("{}", line);
      if line.starts_with("Message: ") {
        return ExecuteRequestResult::OtherError(String::from(&line["Message: ".len()..]));
      }
    }

    return ExecuteRequestResult::OtherError(String::from("Failed to analyze."));
  }
}

fn decide_clear_line(field: &Field) -> i32 {
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

fn analyze_path_nums(stdout: &str) -> String {
  let mut found_paths = vec![];
  let lines = stdout.split("\n").map(|s| s.trim());
  for line in lines {
    if line.starts_with("Found path") {
      found_paths.push(line);
    }
  }

  found_paths.join("\n")
}

fn execute_response(
  out: &ws::Sender,
  request_result: ExecuteRequestResult,
  request: &AnalyzePcMessageReq,
) {
  let res_result = match request_result {
    ExecuteRequestResult::Succeeded(found_paths) => {
      execute_response_succeeced(out, request, found_paths)
    }
    ExecuteRequestResult::OtherError(message) => {
      execute_response_other_error(out, request, message)
    }
  };

  if res_result.is_err() {
    execute_response_other_error(out, request, String::from("Unexpected error occured.")).ok();
  }
}

fn execute_response_succeeced(
  out: &ws::Sender,
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
  out.send(json)?;

  Ok(())
}

fn execute_response_other_error(
  out: &ws::Sender,
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
  out.send(json)?;

  Ok(())
}

fn log(out: &ws::Sender, message: &str) {
  let log = LogMessage::create(message);
  let message = HubMessage::Log(log);

  if let Ok(json) = serde_json::to_string(&message) {
    debug!("response:\n{}", json);
    out.send(json).ok();
  }
}
