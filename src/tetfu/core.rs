use crate::tetsimu2::core::MAX_FIELD_WIDTH;
use crate::tetsimu2::field::Field;

pub const ASCII_TABLE: &str = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
pub const ENCODE_TABLE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
pub const MAX_TETFU_FIELD_HEIGHT: i32 = 23;
pub const MAX_TETFU_FIELD_WIDTH: i32 = MAX_FIELD_WIDTH;
pub const MAX_TETFU_FIELD_SIZE: i32 = MAX_TETFU_FIELD_HEIGHT * MAX_TETFU_FIELD_WIDTH;
pub const MAX_TETFU_FIELD_SIZE_EX: i32 = MAX_TETFU_FIELD_SIZE + MAX_TETFU_FIELD_WIDTH;

#[derive(Debug, Eq, PartialEq)]
pub struct Tetsimu2Content {
  pub field: Field,
  pub comment: String,
}
