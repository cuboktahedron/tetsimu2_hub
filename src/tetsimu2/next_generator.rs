use crate::tetsimu2::core::Tetromino;
use rand::Rng;

pub trait NextGenerator {
  fn next(&mut self) -> Tetromino;
}

pub struct RandomNextGenerator {
  bag: Vec<Tetromino>,
}

impl RandomNextGenerator {
  pub fn new(bag: Vec<Tetromino>) -> RandomNextGenerator {
    RandomNextGenerator { bag }
  }
}

impl NextGenerator for RandomNextGenerator {
  fn next(&mut self) -> Tetromino {
    if self.bag.is_empty() {
      self.bag.push(Tetromino::I);
      self.bag.push(Tetromino::J);
      self.bag.push(Tetromino::L);
      self.bag.push(Tetromino::O);
      self.bag.push(Tetromino::S);
      self.bag.push(Tetromino::T);
      self.bag.push(Tetromino::Z);
    }

    let p = rand::thread_rng().gen_range(0..self.bag.len());
    let tetromino = self.bag.remove(p);
    tetromino
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn next() {
    let mut gen = RandomNextGenerator::new(Vec::new());

    for _ in 0..2 {
      let mut nexts: Vec<Tetromino> = Vec::new();

      for _ in 0..7 {
        nexts.push(gen.next());
      }
      nexts.sort();

      assert_eq!(
        nexts,
        vec![
          Tetromino::I,
          Tetromino::J,
          Tetromino::L,
          Tetromino::O,
          Tetromino::S,
          Tetromino::T,
          Tetromino::Z
        ]
      )
    }
  }
}
