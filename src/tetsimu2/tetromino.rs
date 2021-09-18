use crate::tetsimu2::core::Direction;
use crate::tetsimu2::core::XY;

#[rustfmt::skip]
const SRSS_LEFT: [XY; 16] = [
  ( 1, 0), ( 1,  1), (0, -2), ( 1, -2),
  (-1, 0), (-1, -1), (0,  2), (-1,  2),
  (-1, 0), (-1,  1), (0, -2), (-1, -2),
  ( 1, 0), ( 1, -1), (0,  2), ( 1,  2),
];

#[rustfmt::skip]
const SRSS_RIGHT: [XY; 16] = [
  (-1, 0), (-1,  1), (0, -2), (-1, -2),
  (-1, 0), (-1, -1), (0,  2), (-1,  2),
  ( 1, 0), ( 1,  1), (0, -2), ( 1, -2),
  ( 1, 0), ( 1, -1), (0,  2), ( 1,  2),
];

fn srss_left(direction: Direction) -> &'static [XY] {
  match direction {
    Direction::Up => &SRSS_LEFT[0..4],
    Direction::Left => &SRSS_LEFT[4..8],
    Direction::Down => &SRSS_LEFT[8..12],
    Direction::Right => &SRSS_LEFT[12..16],
  }
}

fn srss_right(direction: Direction) -> &'static [XY] {
  match direction {
    Direction::Up => &SRSS_RIGHT[0..4],
    Direction::Left => &SRSS_RIGHT[4..8],
    Direction::Down => &SRSS_RIGHT[8..12],
    Direction::Right => &SRSS_RIGHT[12..16],
  }
}

pub struct TetrominoI {}
impl TetrominoI {
  #[rustfmt::skip]
  const BLOCKS: [XY; 16] = [
    (0,  0), (-1,  0), (1,  0), ( 2 ,  0),
    (0, -1), ( 0, -2), (0,  0), ( 0 ,  1),
    (1, -1), ( 2, -1), (0, -1), (-1 , -1),
    (1,  0), ( 1,  1), (1, -1), ( 1 , -2),
  ];

  #[rustfmt::skip]
  const SRSS_LEFT: [XY; 16] = [
    (-1, 0), ( 2, 0), (-1,  2), ( 2, -1),
    ( 1, 0), (-2, 0), (-2, -1), ( 1,  2),
    ( 1, 0), (-2, 0), ( 1, -2), (-2,  1),
    ( 2, 0), (-1, 0), ( 2,  1), (-1, -2),
  ];

  #[rustfmt::skip]
  const SRSS_RIGHT: [XY; 16] = [
    (-2, 0), ( 1, 0), (-2, -1), ( 1,  2),
    (-2, 0), ( 1, 0), ( 1, -2), (-2,  1),
    ( 2, 0), (-1, 0), ( 2,  1), (-1, -2),
    (-1, 0), ( 2, 0), (-1,  2), ( 2, -1),
  ];

  pub fn blocks(direction: Direction) -> &'static [XY] {
    match direction {
      Direction::Up => &TetrominoI::BLOCKS[0..4],
      Direction::Left => &TetrominoI::BLOCKS[4..8],
      Direction::Down => &TetrominoI::BLOCKS[8..12],
      Direction::Right => &TetrominoI::BLOCKS[12..16],
    }
  }

  pub fn srss_left(direction: Direction) -> &'static [XY] {
    match direction {
      Direction::Up => &TetrominoI::SRSS_LEFT[0..4],
      Direction::Left => &TetrominoI::SRSS_LEFT[4..8],
      Direction::Down => &TetrominoI::SRSS_LEFT[8..12],
      Direction::Right => &TetrominoI::SRSS_LEFT[12..16],
    }
  }

  pub fn srss_right(direction: Direction) -> &'static [XY] {
    match direction {
      Direction::Up => &TetrominoI::SRSS_RIGHT[0..4],
      Direction::Left => &TetrominoI::SRSS_RIGHT[4..8],
      Direction::Down => &TetrominoI::SRSS_RIGHT[8..12],
      Direction::Right => &TetrominoI::SRSS_RIGHT[12..16],
    }
  }
}

pub struct TetrominoJ {}
impl TetrominoJ {
  #[rustfmt::skip]
  const BLOCKS: [XY; 16] = [
    (0, 0), (-1,   0), ( 1,  0), (-1,  1),
    (0, 0), ( 0 , -1), ( 0,  1), (-1, -1),
    (0, 0), ( 1 ,  0), ( -1, 0), ( 1, -1),
    (0, 0), ( 0 ,  1), ( 0, -1), ( 1,  1),
  ];

  pub fn blocks(direction: Direction) -> &'static [XY] {
    match direction {
      Direction::Up => &TetrominoJ::BLOCKS[0..4],
      Direction::Left => &TetrominoJ::BLOCKS[4..8],
      Direction::Down => &TetrominoJ::BLOCKS[8..12],
      Direction::Right => &TetrominoJ::BLOCKS[12..16],
    }
  }

  pub fn srss_left(direction: Direction) -> &'static [XY] {
    srss_left(direction)
  }

  pub fn srss_right(direction: Direction) -> &'static [XY] {
    srss_right(direction)
  }
}

pub struct TetrominoL {}
impl TetrominoL {
  #[rustfmt::skip]
  const BLOCKS: [XY; 16] = [
    (0, 0), (-1,  0), ( 1,  0), ( 1,  1),
    (0, 0), ( 0, -1), ( 0,  1), (-1,  1),
    (0, 0), ( 1,  0), (-1,  0), (-1, -1),
    (0, 0), ( 0,  1), ( 0, -1), ( 1, -1),
  ];

  pub fn blocks(direction: Direction) -> &'static [XY] {
    match direction {
      Direction::Up => &TetrominoL::BLOCKS[0..4],
      Direction::Left => &TetrominoL::BLOCKS[4..8],
      Direction::Down => &TetrominoL::BLOCKS[8..12],
      Direction::Right => &TetrominoL::BLOCKS[12..16],
    }
  }

  pub fn srss_left(direction: Direction) -> &'static [XY] {
    srss_left(direction)
  }

  pub fn srss_right(direction: Direction) -> &'static [XY] {
    srss_right(direction)
  }
}

pub struct TetrominoO {}
impl TetrominoO {
  #[rustfmt::skip]
  const BLOCKS: [XY; 16] = [
    (0, 0), (0, 1), (1, 1), (1, 0),
    (1, 0), (1, 1), (0, 1), (0, 0),
    (1, 1), (0, 1), (0, 0), (1, 0),
    (0, 1), (0, 0), (1, 0), (1, 1),
  ];

  pub fn blocks(direction: Direction) -> &'static [XY] {
    match direction {
      Direction::Up => &TetrominoO::BLOCKS[0..4],
      Direction::Left => &TetrominoO::BLOCKS[4..8],
      Direction::Down => &TetrominoO::BLOCKS[8..12],
      Direction::Right => &TetrominoO::BLOCKS[12..16],
    }
  }

  pub fn srss_left(direction: Direction) -> &'static [XY] {
    srss_left(direction)
  }

  pub fn srss_right(direction: Direction) -> &'static [XY] {
    srss_right(direction)
  }
}

pub struct TetrominoS {}
impl TetrominoS {
  #[rustfmt::skip]
  const BLOCKS: [XY; 16] = [
    (0, 0), (-1,  0), ( 0,  1), ( 1,  1),
    (0, 0), ( 0, -1), (-1,  0), (-1,  1),
    (0, 0), ( 1,  0), ( 0, -1), (-1, -1),
    (0, 0), ( 0,  1), ( 1,  0), ( 1, -1),
  ];

  pub fn blocks(direction: Direction) -> &'static [XY] {
    match direction {
      Direction::Up => &TetrominoS::BLOCKS[0..4],
      Direction::Left => &TetrominoS::BLOCKS[4..8],
      Direction::Down => &TetrominoS::BLOCKS[8..12],
      Direction::Right => &TetrominoS::BLOCKS[12..16],
    }
  }

  pub fn srss_left(direction: Direction) -> &'static [XY] {
    srss_left(direction)
  }

  pub fn srss_right(direction: Direction) -> &'static [XY] {
    srss_right(direction)
  }
}

pub struct TetrominoT {}
impl TetrominoT {
  #[rustfmt::skip]
  const BLOCKS: [XY; 16] = [
    (0, 0), (-1, 0), (1, 0), (0,  1),
    (0, 0), (-1, 0), (0, 1), (0, -1),
    (0, 0), (-1, 0), (1, 0), (0, -1),
    (0, 0), ( 1, 0), (0, 1), (0, -1),
  ];

  pub fn blocks(direction: Direction) -> &'static [XY] {
    match direction {
      Direction::Up => &TetrominoT::BLOCKS[0..4],
      Direction::Left => &TetrominoT::BLOCKS[4..8],
      Direction::Down => &TetrominoT::BLOCKS[8..12],
      Direction::Right => &TetrominoT::BLOCKS[12..16],
    }
  }

  pub fn srss_left(direction: Direction) -> &'static [XY] {
    srss_left(direction)
  }

  pub fn srss_right(direction: Direction) -> &'static [XY] {
    srss_right(direction)
  }
}

pub struct TetrominoZ {}
impl TetrominoZ {
  #[rustfmt::skip]
  const BLOCKS: [XY; 16] = [
    (0, 0), ( 1,  0), ( 0,  1), (-1,  1),
    (0, 0), ( 0,  1), (-1,  0), (-1, -1),
    (0, 0), (-1,  0), ( 0, -1), ( 1, -1),
    (0, 0), ( 0, -1), ( 1,  0), ( 1,  1),
  ];

  pub fn blocks(direction: Direction) -> &'static [XY] {
    match direction {
      Direction::Up => &TetrominoZ::BLOCKS[0..4],
      Direction::Left => &TetrominoZ::BLOCKS[4..8],
      Direction::Down => &TetrominoZ::BLOCKS[8..12],
      Direction::Right => &TetrominoZ::BLOCKS[12..16],
    }
  }

  pub fn srss_left(direction: Direction) -> &'static [XY] {
    srss_left(direction)
  }

  pub fn srss_right(direction: Direction) -> &'static [XY] {
    srss_right(direction)
  }
}
