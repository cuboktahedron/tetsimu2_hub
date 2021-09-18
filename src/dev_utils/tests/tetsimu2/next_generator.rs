use crate::tetsimu2::core::Tetromino;
use crate::tetsimu2::next_generator::NextGenerator;

pub struct FixedNextGenerator {
  source: Vec<Tetromino>,
}

impl FixedNextGenerator {
  pub fn new(source: Vec<Tetromino>) -> FixedNextGenerator {
    FixedNextGenerator { source }
  }
}

impl NextGenerator for FixedNextGenerator {
  fn next(&mut self) -> Tetromino {
    if self.source.is_empty() {
      panic!("Generator source Exhausted.");
    }

    self.source.remove(0)
  }
}
