use crate::tetfu::core::ASCII_TABLE;
use crate::tetfu::core::ENCODE_TABLE;
use crate::tetfu::core::MAX_TETFU_FIELD_HEIGHT;
use crate::tetfu::core::MAX_TETFU_FIELD_SIZE_EX;
use crate::tetfu::core::MAX_TETFU_FIELD_WIDTH;
use crate::tetsimu2::field::Field;
use num_traits::FromPrimitive;
use substring::Substring;

#[derive(Debug, Eq, PartialEq)]
pub struct Tetsimu2Content {
  pub field: Field,
  pub comment: String,
}

pub struct TetfuDecoder;

impl TetfuDecoder {
  pub fn new() -> TetfuDecoder {
    TetfuDecoder {}
  }

  pub fn decode(&self, tetfu_parameter: String) -> Result<Tetsimu2Content, String> {
    let mut field = Field::new();

    // Skip version identifier.
    let tetfu_parameter = match tetfu_parameter.find('@') {
      Some(p) => tetfu_parameter.substring(p + 1, tetfu_parameter.len()),
      None => &tetfu_parameter,
    };

    let mut dec = vec![];
    for c in tetfu_parameter.chars() {
      match ENCODE_TABLE.find(c) {
        Some(x) => dec.push(x as i32),
        None => (),
      };
    }

    dec.reverse();

    let mut i = 0;
    while i < MAX_TETFU_FIELD_SIZE_EX - 1 {
      let v1 = dec.pop().ok_or("Invalid tetfuParameter passed.")?;
      let v2 = dec.pop().ok_or("Invalid tetfuParameter passed.")?;
      let tmp = v1 + v2 * 64;
      let repeat_cell_count = tmp % MAX_TETFU_FIELD_SIZE_EX;
      let cell = (tmp / MAX_TETFU_FIELD_SIZE_EX) % 17 - 8;
      let cell = FromPrimitive::from_i32(cell).ok_or(format!(
        "Cannot convert cell value({}) to FieldCellValue.",
        cell
      ))?;

      for j in i..(repeat_cell_count + i + 1) {
        let x = j % MAX_TETFU_FIELD_WIDTH;
        let y = j / MAX_TETFU_FIELD_WIDTH;
        let y = MAX_TETFU_FIELD_HEIGHT - y - 1;
        field.set_cell(x, y, cell);
      }

      if tmp == 8 * MAX_TETFU_FIELD_SIZE_EX + 239 {
        dec.pop();
      }

      i += repeat_cell_count + 1;
    }

    let mut comment = String::from("");
    let v1 = dec.pop().ok_or("Invalid tetfuParameter passed.")?;
    let v2 = dec.pop().ok_or("Invalid tetfuParameter passed.")?;
    let v3 = dec.pop().ok_or("Invalid tetfuParameter passed.")?;
    let tmp = v1 + v2 * 64 + v3 * 64 * 64;
    let exists_comment = (tmp / 8 / 4 / MAX_TETFU_FIELD_SIZE_EX / 2 / 2 / 2) % 2 == 1;

    if exists_comment {
      let v1 = dec.pop().ok_or("Invalid tetfuParameter passed.")?;
      let v2 = dec.pop().ok_or("Invalid tetfuParameter passed.")?;
      let comment_len = v1 + v2 * 64;

      let mut i = 0;
      let mut comment_dec = vec![];
      while i < comment_len {
        let v1 = dec.pop().map_or(0, |v| v);
        let v2 = dec.pop().map_or(0, |v| v);
        let v3 = dec.pop().map_or(0, |v| v);
        let v4 = dec.pop().map_or(0, |v| v);
        let v5 = dec.pop().map_or(0, |v| v);

        let mut tmp = v1 + v2 * 64 + v3 * 64 * 64 + v4 * 64 * 64 * 64 + v5 * 64 * 64 * 64 * 64;
        comment_dec.push(ASCII_TABLE.chars().nth((tmp % 96) as usize).unwrap());
        tmp = tmp / 96;
        comment_dec.push(ASCII_TABLE.chars().nth((tmp % 96) as usize).unwrap());
        tmp = tmp / 96;
        comment_dec.push(ASCII_TABLE.chars().nth((tmp % 96) as usize).unwrap());
        tmp = tmp / 96;
        comment_dec.push(ASCII_TABLE.chars().nth((tmp % 96) as usize).unwrap());
        i += 4;
      }

      let decoded_comment = decode(
        comment_dec
          .iter()
          .collect::<String>()
          .substring(0, comment_len as usize),
      )?;
      comment = String::from(decoded_comment);
    }

    Ok(Tetsimu2Content {
      field: field,
      comment: comment,
    })
  }
}

fn decode(s: &str) -> Result<String, String> {
  let mut decoded = vec![];
  let svec: Vec<char> = s.chars().collect();
  let mut i = 0;
  let len = svec.len();
  while i < len {
    let c = svec[i];
    if c == '%' {
      let decoded_char = if svec[i + 1] == 'u' {
        // Surrogate Pair is not supported
        let value = format!(
          "{}{}{}{}",
          svec[i + 2],
          svec[i + 3],
          svec[i + 4],
          svec[i + 5]
        );
        let a = u32::from_str_radix(&value, 16).or_else(|_| Err("Cannot conver value"))?;
        i += 5;
        char::from_u32(a).ok_or(format!("Cannnot conver {} to char", a))?
      } else {
        let value = format!("{}{}", svec[i + 1], svec[i + 2]);
        let a = u32::from_str_radix(&value, 16).or_else(|_| Err("Cannot conver value"))?;
        i += 2;
        char::from_u32(a).ok_or(format!("Cannnot conver {} to char", a))?
      };

      decoded.push(decoded_char);
    } else {
      decoded.push(c);
    }

    i = i + 1;
  }

  Ok(decoded.into_iter().collect())
}

#[cfg(test)]
mod tests_fn {
  use super::*;

  #[test]
  fn decode_test() {
    assert_eq!(decode(
      "%20%21%22%23%24%25%26%27%28%29*+%2C-./0123456789%3A%3B%3C%3D%3E%3F@ABCDEFGHIJKLMNOPQRSTUVWXYZ%5B%5C%5D%5E_%60abcdefghijklmnopqrstuvwxyz%7B%7C%7D%7E"),
      Ok(String::from(" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~")));

    assert_eq!(decode("%u3042"), Ok(String::from("あ")));
    assert_eq!(
      decode("abc%21%23%u3042%u3044%u3046"),
      Ok(String::from("abc!#あいう"))
    );
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::tetsimu2::core::FieldCellValue;

  #[test]
  fn decode() {
    let field = Field::new();
    let tetfu = TetfuDecoder::new();
    assert_eq!(
      tetfu.decode(String::from("v115@vhAAAA")),
      Ok(Tetsimu2Content {
        field: field,
        comment: String::from(""),
      })
    );

    let mut field = Field::new();
    field.set_cell(9, 0, FieldCellValue::I);
    assert_eq!(
      tetfu.decode(String::from("v115@khwhJeAAA")),
      Ok(Tetsimu2Content {
        field: field,
        comment: String::from("")
      }),
    );

    let mut field = Field::new();
    field.set_cell(9, 0, FieldCellValue::I);
    assert_eq!(
      tetfu.decode(String::from("v115@khwhJeAAPHADHnGEF2+CA")),
      Ok(Tetsimu2Content {
        field: field,
        comment: String::from("Comment")
      }),
    );

    let field = Field::new();
    assert_eq!(
      tetfu.decode(String::from("v115@vhAAAPTCFbcRAyp78AynwABFblRAyv78A2nQOBFbuR?AyFflAFLHtAuW85AyclHB2iOVBlsCSATDUABD4K6BlsLSAT?5gwBC1J+BG7yLCKBcZCOHFnCSNu0CWTXCDaYfzBlPHSAVGE?HBFvcKBwBekDkIHyDoOw/DsUZNEwaCbE0groE4mEUAXD0NB?D4T6BlyTBA")),
      Ok(Tetsimu2Content {
        field: field,
        comment: String::from(" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~")
      }),
    );
  }
}
