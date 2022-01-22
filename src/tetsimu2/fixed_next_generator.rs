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
  fn next(&mut self) -> Option<Tetromino> {
    if self.source.is_empty() {
      return None;
    }

    Some(self.source.remove(0))
  }

  fn has_next(&self) -> bool {
    !self.source.is_empty()
  }
}
