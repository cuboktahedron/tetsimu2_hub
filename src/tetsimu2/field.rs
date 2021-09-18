use crate::tetsimu2::core::FieldCellValue;
use crate::tetsimu2::core::MAX_FIELD_HEIGHT;
use crate::tetsimu2::core::MAX_FIELD_SIZE;
use crate::tetsimu2::core::MAX_FIELD_WIDTH;
use crate::tetsimu2::core::MAX_INNER_FIELD_HEIGHT;
use crate::tetsimu2::current_tetromino::CurrentTetromino;
use core::fmt::Debug;

#[derive(Eq, PartialEq, Clone)]
pub struct Field {
  pub data: [FieldCellValue; 300],
}

impl Field {
  pub fn new() -> Field {
    Field {
      data: [FieldCellValue::None; MAX_FIELD_SIZE as usize],
    }
  }

  pub fn set_cell(&mut self, x: i32, y: i32, value: FieldCellValue) {
    if x < 0 || x >= MAX_FIELD_WIDTH {
      return;
    }

    if y < 0 || y >= MAX_FIELD_HEIGHT {
      return;
    }

    let p: usize = (y * MAX_FIELD_WIDTH + x) as usize;
    if p < 300 {
      self.data[p] = value
    }
  }

  pub fn get_cell(&self, x: i32, y: i32) -> FieldCellValue {
    if x < 0 || x >= MAX_FIELD_WIDTH {
      return FieldCellValue::Garbage;
    }

    if y < 0 || y >= MAX_FIELD_HEIGHT {
      return FieldCellValue::Garbage;
    }

    let p: usize = (y * MAX_FIELD_WIDTH + x) as usize;
    if p < 300 {
      self.data[p]
    } else {
      FieldCellValue::Garbage
    }
  }

  pub fn is_overlapped(&self, tetromino: &CurrentTetromino) -> bool {
    tetromino
      .blocks()
      .iter()
      .any(|&(x, y)| self.get_cell(x, y) != FieldCellValue::None)
  }

  pub fn is_in_inner_field(&self, tetromino: &CurrentTetromino) -> bool {
    tetromino
      .blocks()
      .iter()
      .all(|&(_, y)| y < MAX_INNER_FIELD_HEIGHT)
  }

  pub fn can_settle_tetromino(&self, tetromino: &CurrentTetromino) -> bool {
    let mut current = tetromino.clone();
    current.drop_to_bottom(&self);

    *tetromino == current
  }

  pub fn settle_tetromino(&mut self, tetromino: &CurrentTetromino) {
    for (x, y) in tetromino.blocks() {
      self.set_cell(x, y, FieldCellValue::from(tetromino.r#type));
    }
  }

  pub fn clear_lines(&mut self) -> u32 {
    let mut after_data = [FieldCellValue::None; 300];
    let mut after_y = 0;
    let mut cleared_line = 0;

    for y in 0..MAX_FIELD_HEIGHT {
      let mut is_cleared_line = true;

      for x in 0..MAX_FIELD_WIDTH {
        let p = (y * MAX_FIELD_WIDTH + x) as usize;
        if self.data[p] == FieldCellValue::None {
          is_cleared_line = false;
          break;
        }
      }

      if is_cleared_line {
        cleared_line += 1;
      } else {
        for x in 0..MAX_FIELD_WIDTH {
          let p = (y * MAX_FIELD_WIDTH + x) as usize;
          let after_p = (after_y * MAX_FIELD_WIDTH + x) as usize;
          after_data[after_p] = self.data[p];
        }
        after_y += 1;
      }
    }

    self.data = after_data;
    cleared_line
  }
}

impl Debug for Field {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    writeln!(f, "")?;

    for y in 0..MAX_FIELD_HEIGHT {
      let y = MAX_FIELD_HEIGHT - y - 1;
      write!(f, "{:02}: ", y)?;
      for x in 0..MAX_FIELD_WIDTH {
        let cell = self.get_cell(x, y);
        write!(f, "{}", cell as i32)?;
      }
      writeln!(f, "")?;
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::dev_utils::tests::tetsimu2::field::make_field;

  #[test]
  fn cell() {
    let mut field = Field::new();
    field.set_cell(5, 1, FieldCellValue::I);

    assert_eq!(field.get_cell(0, 0), FieldCellValue::None);
    assert_eq!(field.get_cell(-1, 0), FieldCellValue::Garbage);
    assert_eq!(field.get_cell(MAX_FIELD_WIDTH, 0), FieldCellValue::Garbage);
    assert_eq!(field.get_cell(0, MAX_FIELD_HEIGHT), FieldCellValue::Garbage);
    assert_eq!(field.get_cell(0, -1), FieldCellValue::Garbage);
    assert_eq!(field.get_cell(5, 1), FieldCellValue::I);
  }

  #[test]
  fn claer_lines_should_not_clear_lines() {
    #[rustfmt::skip]
    let mut field = make_field("NGGGGGGGGG");

    assert_eq!(field.clear_lines(), 0);
    assert_eq!(field, make_field("NGGGGGGGGG"));
  }

  #[test]
  fn claer_lines_should_clear_lines() {
    #[rustfmt::skip]
    let mut field = make_field(&format!("{}{}{}{}{}", 
      "GGGGGGGGGG",
      "GGGGGGGNGG",
      "GGGGGGGGGG",
      "GGGGGGGGNG",
      "GGGGGGGGGG"
    ));

    assert_eq!(field.clear_lines(), 3);
    #[rustfmt::skip]
    assert_eq!(field, make_field(&format!("{}{}", 
      "GGGGGGGNGG",
      "GGGGGGGGNG",
    )));
  }
}
