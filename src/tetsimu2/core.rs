use core::fmt::Debug;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

pub const MAX_FIELD_WIDTH: i32 = 10;
pub const MAX_FIELD_HEIGHT: i32 = 30;
pub const MAX_INNER_FIELD_HEIGHT: i32 = 20;
pub const MAX_FIELD_SIZE: i32 = 10 * 30;

pub type XY = (i32, i32);

#[derive(Debug, Eq, PartialEq, Clone, Copy, FromPrimitive)]
pub enum Direction {
  Up = 0,
  Left,
  Down,
  Right,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, FromPrimitive, PartialOrd, Ord)]
pub enum Tetromino {
  I = 1,
  J = 2,
  L = 3,
  O = 4,
  S = 5,
  T = 6,
  Z = 7,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Step {
  pub r#type: Tetromino,
  pub dir: Direction,
  pub x: u8,
  pub y: u8,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, FromPrimitive)]
pub enum FieldCellValue {
  None = 0,
  I,
  J,
  L,
  O,
  S,
  T,
  Z,
  Garbage,
}

impl From<Tetromino> for FieldCellValue {
  fn from(r#type: Tetromino) -> Self {
    let t = r#type as i32;
    FromPrimitive::from_i32(t).unwrap()
  }
}
