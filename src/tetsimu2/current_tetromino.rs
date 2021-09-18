use crate::tetsimu2::core::Direction;
use crate::tetsimu2::core::Tetromino;
use crate::tetsimu2::core::XY;
use crate::tetsimu2::field::Field;
use crate::tetsimu2::tetromino::TetrominoI;
use crate::tetsimu2::tetromino::TetrominoJ;
use crate::tetsimu2::tetromino::TetrominoL;
use crate::tetsimu2::tetromino::TetrominoO;
use crate::tetsimu2::tetromino::TetrominoS;
use crate::tetsimu2::tetromino::TetrominoT;
use crate::tetsimu2::tetromino::TetrominoZ;
use num_traits::FromPrimitive;
use std::cmp::Ordering;
use std::cmp::Ordering::Equal;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct CurrentTetromino {
  pub r#type: Tetromino,
  pub direction: Direction,
  pub x: i32,
  pub y: i32,
}

impl CurrentTetromino {
  pub fn blocks(&self) -> Vec<XY> {
    let blocks = match self.r#type {
      Tetromino::I => TetrominoI::blocks(self.direction),
      Tetromino::J => TetrominoJ::blocks(self.direction),
      Tetromino::L => TetrominoL::blocks(self.direction),
      Tetromino::O => TetrominoO::blocks(self.direction),
      Tetromino::S => TetrominoS::blocks(self.direction),
      Tetromino::T => TetrominoT::blocks(self.direction),
      Tetromino::Z => TetrominoZ::blocks(self.direction),
    };

    blocks
      .iter()
      .map(|(x, y)| (self.x + x, self.y + y))
      .collect()
  }

  pub fn srss_left(&self) -> &'static [XY] {
    match self.r#type {
      Tetromino::I => TetrominoI::srss_left(self.direction),
      Tetromino::J => TetrominoJ::srss_left(self.direction),
      Tetromino::L => TetrominoL::srss_left(self.direction),
      Tetromino::O => TetrominoO::srss_left(self.direction),
      Tetromino::S => TetrominoS::srss_left(self.direction),
      Tetromino::T => TetrominoT::srss_left(self.direction),
      Tetromino::Z => TetrominoZ::srss_left(self.direction),
    }
  }

  pub fn srss_right(&self) -> &'static [XY] {
    match self.r#type {
      Tetromino::I => TetrominoI::srss_right(self.direction),
      Tetromino::J => TetrominoJ::srss_right(self.direction),
      Tetromino::L => TetrominoL::srss_right(self.direction),
      Tetromino::O => TetrominoO::srss_right(self.direction),
      Tetromino::S => TetrominoS::srss_right(self.direction),
      Tetromino::T => TetrominoT::srss_right(self.direction),
      Tetromino::Z => TetrominoZ::srss_right(self.direction),
    }
  }

  pub fn drop_to_bottom(&mut self, field: &Field) {
    let mut current = self.clone();
    loop {
      current.y -= 1;
      if field.is_overlapped(&current) {
        current.y += 1;
        break;
      }
    }

    self.y = current.y;
  }

  pub fn soft_drop(&mut self, field: &Field) -> bool {
    let mut current = self.clone();
    current.y -= 1;
    if !field.is_overlapped(&current) {
      self.y -= 1;
      true
    } else {
      false
    }
  }

  pub fn move_left(&mut self, field: &Field) -> bool {
    let mut current = self.clone();
    current.x -= 1;
    if !field.is_overlapped(&current) {
      self.x -= 1;
      true
    } else {
      false
    }
  }

  pub fn move_right(&mut self, field: &Field) -> bool {
    let mut current = self.clone();
    current.x += 1;
    if !field.is_overlapped(&current) {
      self.x += 1;
      true
    } else {
      false
    }
  }

  pub fn turn_left(&mut self, field: &Field) -> bool {
    let mut current = self.clone();

    match current.direction {
      Direction::Up => current.direction = Direction::Left,
      Direction::Left => current.direction = Direction::Down,
      Direction::Down => current.direction = Direction::Right,
      Direction::Right => current.direction = Direction::Up,
    }

    if field.is_overlapped(&current) {
      let srs = self.srss_left();
      let org_current = current;
      for i in 0..4 {
        let mut current = org_current.clone();
        let (dx, dy) = srs[i];
        current.x += dx;
        current.y += dy;

        if !field.is_overlapped(&current) {
          self.x = current.x;
          self.y = current.y;
          self.direction = current.direction;
          return true;
        }
      }
      return false;
    } else {
      self.direction = current.direction;
      return true;
    }
  }

  pub fn turn_right(&mut self, field: &Field) -> bool {
    let mut current = self.clone();

    match current.direction {
      Direction::Up => current.direction = Direction::Right,
      Direction::Left => current.direction = Direction::Up,
      Direction::Down => current.direction = Direction::Left,
      Direction::Right => current.direction = Direction::Down,
    }

    if field.is_overlapped(&current) {
      let srs = self.srss_right();
      let org_current = current;
      for i in 0..4 {
        let mut current = org_current.clone();
        let (dx, dy) = srs[i];
        current.x += dx;
        current.y += dy;

        if !field.is_overlapped(&current) {
          self.x = current.x;
          self.y = current.y;
          self.direction = current.direction;
          return true;
        }
      }
      return false;
    } else {
      self.direction = current.direction;
      return true;
    }
  }

  pub fn flip(&mut self) -> bool {
    if self.r#type == Tetromino::J {
      return false;
    }

    if self.r#type == Tetromino::L {
      return false;
    }

    if self.r#type == Tetromino::T {
      return false;
    }

    if self.r#type == Tetromino::O {
      return true;
    }

    let mut current = self.clone();
    current.direction = FromPrimitive::from_i32((self.direction as i32 + 2) % 4).unwrap();

    let mut blocks1 = self.blocks();
    let mut blocks2 = current.blocks();
    blocks1.sort_by(CurrentTetromino::compare_blocks);
    blocks2.sort_by(CurrentTetromino::compare_blocks);

    let (bx1, by1) = blocks1[0];
    let (bx2, by2) = blocks2[0];
    self.x -= bx2 - bx1;
    self.y -= by2 - by1;
    self.direction = current.direction;

    true
  }

  fn compare_blocks(b1: &(i32, i32), b2: &(i32, i32)) -> Ordering {
    let (x1, y1) = b1;
    let (x2, y2) = b2;

    if x1.cmp(x2) != Equal {
      x1.cmp(x2)
    } else {
      y1.cmp(y2)
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::dev_utils::tests::tetsimu2::field::make_field;
  use crate::tetsimu2::core::Tetromino;
  use parameterized::parameterized;

  #[test]
  fn drop_to_bottom() {
    let field = Field::new();
    let mut current = CurrentTetromino {
      r#type: Tetromino::T,
      direction: Direction::Up,
      x: 4,
      y: 5,
    };

    current.drop_to_bottom(&field);
    assert_eq!(current.y, 0);
  }

  #[test]
  fn soft_drop() {
    let field = Field::new();
    let mut current = CurrentTetromino {
      r#type: Tetromino::T,
      direction: Direction::Up,
      x: 4,
      y: 5,
    };

    assert_eq!(current.soft_drop(&field), true);
    assert_eq!(current.y, 4);
  }

  #[test]
  fn move_left() {
    let field = Field::new();
    let mut current = CurrentTetromino {
      r#type: Tetromino::T,
      direction: Direction::Up,
      x: 4,
      y: 5,
    };

    assert_eq!(current.move_left(&field), true);
    assert_eq!(current.x, 3);
  }

  #[test]
  fn move_right() {
    let field = Field::new();
    let mut current = CurrentTetromino {
      r#type: Tetromino::T,
      direction: Direction::Up,
      x: 4,
      y: 5,
    };

    assert_eq!(current.move_right(&field), true);
    assert_eq!(current.x, 5);
  }

  #[test]
  fn turn_left() {
    let field = Field::new();
    let mut current = CurrentTetromino {
      r#type: Tetromino::T,
      direction: Direction::Up,
      x: 4,
      y: 5,
    };

    assert_eq!(current.turn_left(&field), true);
    assert_eq!(current.direction, Direction::Left);

    assert_eq!(current.turn_left(&field), true);
    assert_eq!(current.direction, Direction::Down);

    assert_eq!(current.turn_left(&field), true);
    assert_eq!(current.direction, Direction::Right);

    assert_eq!(current.turn_left(&field), true);
    assert_eq!(current.direction, Direction::Up);
  }

  #[test]
  fn turn_left_with_srs_not_i() {
    #[rustfmt::skip]
    let field = make_field(
      &format!("{}{}{}{}{}",
          "NNNNNNNNGG",
          "NNNNNNNNNG",
          "GGGGGGGGNG",
          "GGGGGGGNNG",
          "GGGGGGGGNG"));

    let mut current = CurrentTetromino {
      r#type: Tetromino::T,
      direction: Direction::Up,
      x: 7,
      y: 3,
    };

    assert_eq!(current.turn_left(&field), true);
    assert_eq!(current.direction, Direction::Left);
    assert_eq!(current.x, 8);
    assert_eq!(current.y, 1);
  }

  #[test]
  fn turn_left_with_srs_i() {
    #[rustfmt::skip]
    let field = make_field(
      &format!("{}{}{}{}{}{}",
        "NNNNNNGNNG",
        "NNNNNNNNNN",
        "GGGGGGNNGN",
        "GGGGGGGGGN",
        "GGGGGGGGGN",
        "GGGGGGGGGN"));

    let mut current = CurrentTetromino {
      r#type: Tetromino::I,
      direction: Direction::Up,
      x: 7,
      y: 4,
    };

    assert_eq!(current.turn_left(&field), true);
    assert_eq!(current.direction, Direction::Left);
    assert_eq!(current.x, 9);
    assert_eq!(current.y, 3);
  }

  #[test]
  fn turn_right() {
    let field = Field::new();
    let mut current = CurrentTetromino {
      r#type: Tetromino::T,
      direction: Direction::Up,
      x: 4,
      y: 5,
    };

    assert_eq!(current.turn_right(&field), true);
    assert_eq!(current.direction, Direction::Right);
    assert_eq!(current.turn_right(&field), true);
    assert_eq!(current.direction, Direction::Down);
    assert_eq!(current.turn_right(&field), true);
    assert_eq!(current.direction, Direction::Left);
    assert_eq!(current.turn_right(&field), true);
    assert_eq!(current.direction, Direction::Up);
  }

  #[test]
  fn turn_right_with_srs_not_i() {
    #[rustfmt::skip]
    let field = make_field(
      &format!("{}{}{}{}{}",
          "GGNNNNNNNN",
          "GNNNNNNNNN",
          "GNGGGGGGGG",
          "GNNGGGGGGG",
          "GNGGGGGGGG"));

    let mut current = CurrentTetromino {
      r#type: Tetromino::T,
      direction: Direction::Up,
      x: 2,
      y: 3,
    };

    assert_eq!(current.turn_right(&field), true);
    assert_eq!(current.direction, Direction::Right);
    assert_eq!(current.x, 1);
    assert_eq!(current.y, 1);
  }

  #[test]
  fn turn_right_with_srs_i() {
    #[rustfmt::skip]
    let field = make_field(
      &format!("{}{}{}{}{}{}",
        "GNNNNNNNNN",
        "NNNNNNNNNN",
        "GGGGNNNNNN",
        "NNGGGGGGGG",
        "GNGGGGGGGG",
        "NGGGGGGGGG"));

    let mut current = CurrentTetromino {
      r#type: Tetromino::I,
      direction: Direction::Up,
      x: 1,
      y: 4,
    };

    assert_eq!(current.turn_right(&field), true);
    assert_eq!(current.direction, Direction::Right);
    assert_eq!(current.x, 2);
    assert_eq!(current.y, 6);
  }

  #[parameterized(r#type = {
    Tetromino::J, Tetromino::L, Tetromino::O, Tetromino::T
  }, expected = {
    false, false, true, false
  })]
  fn flip_jlot(r#type: Tetromino, expected: bool) {
    #[rustfmt::skip]

    let mut current = CurrentTetromino {
      r#type,
      direction: Direction::Up,
      x: 5,
      y: 5,
    };

    assert_eq!(current.flip(), expected);
  }

  #[parameterized(direction = {
    Direction::Up, Direction::Left, Direction::Right, Direction::Down
  })]
  fn flip_i(direction: Direction) {
    #[rustfmt::skip]

    let mut current = CurrentTetromino {
      r#type: Tetromino::I,
      direction,
      x: 5,
      y: 5,
    };
    let mut before_blocks = current.blocks();
    assert_eq!(current.flip(), true);

    let mut after_blocks = current.blocks();
    before_blocks.sort_by(CurrentTetromino::compare_blocks);
    after_blocks.sort_by(CurrentTetromino::compare_blocks);
    assert_eq!(after_blocks, before_blocks);
  }

  #[parameterized(direction = {
    Direction::Up, Direction::Left, Direction::Right, Direction::Down
  })]
  fn flip_s(direction: Direction) {
    #[rustfmt::skip]

    let mut current = CurrentTetromino {
      r#type: Tetromino::S,
      direction,
      x: 5,
      y: 5,
    };
    let mut before_blocks = current.blocks();
    assert_eq!(current.flip(), true);

    let mut after_blocks = current.blocks();
    before_blocks.sort_by(CurrentTetromino::compare_blocks);
    after_blocks.sort_by(CurrentTetromino::compare_blocks);
    assert_eq!(after_blocks, before_blocks);
  }

  #[parameterized(direction = {
    Direction::Up, Direction::Left, Direction::Right, Direction::Down
  })]
  fn flip_z(direction: Direction) {
    #[rustfmt::skip]

    let mut current = CurrentTetromino {
      r#type: Tetromino::Z,
      direction,
      x: 5,
      y: 5,
    };
    let mut before_blocks = current.blocks();
    assert_eq!(current.flip(), true);

    let mut after_blocks = current.blocks();
    before_blocks.sort_by(CurrentTetromino::compare_blocks);
    after_blocks.sort_by(CurrentTetromino::compare_blocks);
    assert_eq!(after_blocks, before_blocks);
  }
}
