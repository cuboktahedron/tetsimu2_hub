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
  let request_result = execute_request(&message, &settings.solution_finder);
  execute_response(out, request_result, &message);
}

fn execute_request(
  message: &AnalyzePcMessageReq,
  settings: &SolutionFinderSettings,
) -> ExecuteRequestResult {
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

  let tetfu = tetfu_encoder.encode(&Tetsimu2Content {
    field: Field { data },
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
    warn!("stderr:\n{}", String::from_utf8_lossy(&output.stderr));

    let err_message = String::from_utf8_lossy(&output.stderr);
    return ExecuteRequestResult::OtherError(String::from(err_message));
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
  let response = AnalyzePcMessageRes {
    header: HubMessageResHeader {
      message_id: Uuid::new_v4().to_string(),
      request_message_id: request.header.message_id.clone(),
      result: AnalyzePcMessageResResult::Succeeded as i32,
    },
    body: AnalyzePcMessageResBody {
      message: found_paths,
    },
  };

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
  let response = AnalyzePcMessageRes {
    header: HubMessageResHeader {
      message_id: Uuid::new_v4().to_string(),
      request_message_id: request.header.message_id.clone(),
      result: AnalyzePcMessageResResult::Succeeded as i32,
    },
    body: AnalyzePcMessageResBody {
      message: String::from(message),
    },
  };

  let json = serde_json::to_string(&response)?;
  debug!("response:\n{}", json);
  out.send(json)?;

  Ok(())
}
