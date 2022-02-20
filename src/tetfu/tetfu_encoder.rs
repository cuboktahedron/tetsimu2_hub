use crate::tetfu::core::Tetsimu2Content;
use crate::tetfu::core::ASCII_TABLE;
use crate::tetfu::core::ENCODE_TABLE;
use crate::tetfu::core::MAX_TETFU_FIELD_HEIGHT;
use crate::tetfu::core::MAX_TETFU_FIELD_SIZE;
use crate::tetfu::core::MAX_TETFU_FIELD_SIZE_EX;
use crate::tetsimu2::core::FieldCellValue;
use crate::tetsimu2::core::MAX_FIELD_WIDTH;
use crate::tetsimu2::field::Field;
use substring::Substring;

pub struct TetfuEncoder;

impl TetfuEncoder {
  pub fn new() -> TetfuEncoder {
    TetfuEncoder {}
  }

  pub fn encode(&self, content: &Tetsimu2Content) -> String {
    let mut enc = vec![];

    let last_encode_value = self.encode_for_field(&content.field, &mut enc);

    let same_as_previous_page = 8 * MAX_TETFU_FIELD_SIZE_EX + 239;
    if last_encode_value == same_as_previous_page {
      // Encoding does not take a page into account. So repeat count is fixed at 0.
      enc.push(0);
    }

    if content.comment == "" {
      // If there is no comment, the data is not related to tetsimu2.
      // So just set zero.
      enc.push(0);
      enc.push(0);
      enc.push(0);
    } else {
      // If there is a comment, set the information that there is a comment and comment data.
      let mut tmp = 2 * 2 * 2 * MAX_TETFU_FIELD_SIZE_EX * 4 * 8;
      enc.push(tmp % 64);
      tmp = tmp / 64;
      enc.push(tmp % 64);
      tmp = tmp / 64;
      enc.push(tmp % 64);

      self.encode_for_comment(&content.comment, &mut enc);
    }

    let mut enc_result = vec![];

    for (i, c) in enc.iter().enumerate() {
      enc_result.push(ENCODE_TABLE.chars().nth(*c as usize).unwrap());
      if i % 47 == 41 {
        enc_result.push('?')
      }
    }
    let encoded = enc_result.iter().cloned().collect::<String>();
    format!("{}{}", "v115@".to_string(), encoded)
  }

  fn encode_for_field(&self, field: &Field, enc: &mut Vec<i32>) -> i32 {
    let mut repeat_cell_count = 0;
    let mut prev_cell = self.convert_cell(field.get_cell(0, MAX_TETFU_FIELD_HEIGHT - 1)) + 8;

    for p in 1..(MAX_TETFU_FIELD_SIZE_EX) {
      let cell = if p >= MAX_TETFU_FIELD_SIZE {
        FieldCellValue::None as i32 + 8
      } else {
        let x = p % MAX_FIELD_WIDTH;
        let y = (MAX_TETFU_FIELD_HEIGHT - 1) - (p / MAX_FIELD_WIDTH); // 0 ～ MAX_TETFU_FIELD_HEIGHT
        self.convert_cell(field.get_cell(x, y)) + 8
      };
      if cell != prev_cell {
        let tmp = prev_cell * MAX_TETFU_FIELD_SIZE_EX + repeat_cell_count;
        enc.push(tmp % 64);
        enc.push(tmp / 64);

        repeat_cell_count = 0;
      } else if p == MAX_TETFU_FIELD_SIZE_EX - 1 {
        let tmp = prev_cell * MAX_TETFU_FIELD_SIZE_EX + repeat_cell_count + 1;
        enc.push(tmp % 64);
        enc.push(tmp / 64);

        return tmp;
      } else {
        repeat_cell_count += 1;
      }
      prev_cell = cell;
    }

    panic!("This is bug.");
  }

  fn encode_for_comment(&self, comment: &str, enc: &mut Vec<i32>) {
    // Not perfect as it behaves differently from javascript 'escape' function.
    let escaped_comment = encode(comment);
    let espaced_comment = escaped_comment.substring(0, 4096);

    let comment_len = espaced_comment.len() as i32;
    let mut tmp = comment_len;

    enc.push(tmp % 64);
    tmp = tmp / 64;
    enc.push(tmp % 64);

    let f = |i: i32| -> i32 {
      let c = match espaced_comment.chars().nth((i) as usize) {
        Some(x) => x,
        None => return 0,
      };
      let value = ASCII_TABLE.find(c).unwrap();
      value as i32
    };

    let mut i = 0;
    while i < comment_len {
      let mut tmp = f(i);
      tmp += f(i + 1) * 96;
      tmp += f(i + 2) * 96 * 96;
      tmp += f(i + 3) * 96 * 96 * 96;

      enc.push(tmp % 64);
      tmp = tmp / 64;
      enc.push(tmp % 64);
      tmp = tmp / 64;
      enc.push(tmp % 64);
      tmp = tmp / 64;
      enc.push(tmp % 64);
      tmp = tmp / 64;
      enc.push(tmp % 64);
      i += 4;
    }
  }

  fn convert_cell(&self, cell: FieldCellValue) -> i32 {
    match cell {
      FieldCellValue::None => 0,
      FieldCellValue::I => 1,
      FieldCellValue::J => 6,
      FieldCellValue::L => 2,
      FieldCellValue::O => 3,
      FieldCellValue::S => 7,
      FieldCellValue::T => 5,
      FieldCellValue::Z => 4,
      FieldCellValue::Garbage => 8,
    }
  }
}

fn encode(s: &str) -> String {
  s.chars().map(encode_char).collect()
}

fn encode_char(c: char) -> String {
  if c == '*' || c == '+' || c == '-' || c == '.' || c == '/' || c == '@' || c == '_' {
    c.to_string()
  } else if '0' <= c && c <= '9' {
    c.to_string()
  } else if 'A' <= c && c <= 'Z' {
    c.to_string()
  } else if 'a' <= c && c <= 'z' {
    c.to_string()
  } else {
    let encoded = format!("%{:X}", c as u32);
    if encoded.len() == 3 {
      encoded
    } else {
      format!("%u{}", encoded.chars().skip(1).collect::<String>())
    }
  }
}

#[cfg(test)]
mod tests_fn {
  use super::*;

  #[test]
  fn encode_test() {
    assert_eq!(encode(
      " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~"),
      "%20%21%22%23%24%25%26%27%28%29*+%2C-./0123456789%3A%3B%3C%3D%3E%3F@ABCDEFGHIJKLMNOPQRSTUVWXYZ%5B%5C%5D%5E_%60abcdefghijklmnopqrstuvwxyz%7B%7C%7D%7E");

    assert_eq!(encode("あ"), "%u3042");
    assert_eq!(encode("abc!#あいう"), "abc%21%23%u3042%u3044%u3046");
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn encode() {
    let field = Field::new();
    let tetfu = TetfuEncoder::new();
    assert_eq!(
      tetfu.encode(&Tetsimu2Content {
        field: field,
        comment: String::from(""),
      }),
      "v115@vhAAAA"
    );

    let mut field = Field::new();
    field.set_cell(9, 0, FieldCellValue::I);
    assert_eq!(
      tetfu.encode(&Tetsimu2Content {
        field: field,
        comment: String::from("")
      }),
      "v115@khwhJeAAA"
    );

    let mut field = Field::new();
    field.set_cell(9, 0, FieldCellValue::I);
    assert_eq!(
      tetfu.encode(&Tetsimu2Content {
        field: field,
        comment: String::from("Comment"),
      }),
      "v115@khwhJeAAPHADHnGEF2+CA"
    );

    let mut field = Field::new();
    field.set_cell(0, 0, FieldCellValue::None);
    field.set_cell(1, 0, FieldCellValue::I);
    field.set_cell(2, 0, FieldCellValue::J);
    field.set_cell(3, 0, FieldCellValue::L);
    field.set_cell(4, 0, FieldCellValue::O);
    field.set_cell(5, 0, FieldCellValue::S);
    field.set_cell(6, 0, FieldCellValue::T);
    field.set_cell(7, 0, FieldCellValue::Z);
    field.set_cell(8, 0, FieldCellValue::Garbage);
    assert_eq!(
      tetfu.encode(&Tetsimu2Content {
        field: field,
        comment: String::from("Comment"),
      }),
      "v115@chwhg0glQpQ4wwAtA8KeAAPHADHnGEF2+CA"
    );

    let field = Field::new();
    assert_eq!(tetfu.encode(&Tetsimu2Content{field: field, comment: String::from(" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~")}),
      "v115@vhAAAPTCFbcRAyp78AynwABFblRAyv78A2nQOBFbuR?AyFflAFLHtAuW85AyclHB2iOVBlsCSATDUABD4K6BlsLSAT?5gwBC1J+BG7yLCKBcZCOHFnCSNu0CWTXCDaYfzBlPHSAVGE?HBFvcKBwBekDkIHyDoOw/DsUZNEwaCbE0groE4mEUAXD0NB?D4T6BlyTBA");
  }
}
