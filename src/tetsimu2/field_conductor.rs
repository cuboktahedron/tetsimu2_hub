use crate::tetsimu2::core::Direction;
use crate::tetsimu2::core::Tetromino;
use crate::tetsimu2::current_tetromino::CurrentTetromino;
use crate::tetsimu2::field::Field;
use crate::tetsimu2::hold::Hold;
use crate::tetsimu2::next_generator::NextGenerator;

pub struct FieldConductor {
  current: CurrentTetromino,
  next_generator: Box<dyn NextGenerator>,
  hold: Hold,
  is_dead: bool,
  field: Field,
}

impl FieldConductor {
  pub fn new(mut gen: Box<dyn NextGenerator>) -> FieldConductor {
    let current = CurrentTetromino {
      r#type: gen.next().expect("Cannot generate next."),
      direction: Direction::Up,
      x: 4,
      y: 19,
    };

    FieldConductor {
      current,
      is_dead: false,
      field: Field::new(),
      hold: Hold::new(),
      next_generator: gen,
    }
  }

  pub fn can_hold(&self) -> bool {
    self.hold.can_hold()
  }

  pub fn holded(&self) -> Option<Tetromino> {
    self.hold.holded()
  }

  pub fn is_dead(&self) -> bool {
    self.is_dead
  }

  pub fn hard_drop(&mut self) {
    self.current.drop_to_bottom(&self.field);
    self.field.settle_tetromino(&self.current);

    if !self.field.is_in_inner_field(&self.current) {
      self.is_dead = true;
      return;
    }

    self.field.clear_lines();

    if !self.proceed_next() {
      self.is_dead = true;
    }
  }

  pub fn soft_drop(&mut self) -> bool {
    self.current.soft_drop(&self.field)
  }

  pub fn move_left(&mut self) -> bool {
    self.current.move_left(&self.field)
  }

  pub fn move_right(&mut self) -> bool {
    self.current.move_right(&self.field)
  }

  pub fn hold(&mut self) -> bool {
    match self.hold.hold(self.current.r#type) {
      Ok(holded) => {
        match holded {
          Some(r#type) => self.current.r#type = r#type,
          None => {
            self.proceed_next();
          }
        }
        true
      }
      Err(_) => false,
    }
  }

  fn proceed_next(&mut self) -> bool {
    let mut current = CurrentTetromino {
      r#type: self.next_generator.next().expect("Cannot generate next."),
      direction: Direction::Up,
      x: 4,
      y: 19,
    };

    if !self.field.is_overlapped(&current) {
      self.current = current;
      true
    } else {
      current.y += 1;
      if !self.field.is_overlapped(&current) {
        self.current = current;
        true
      } else {
        false
      }
    }
  }

  pub fn reset(&mut self, gen: Box<dyn NextGenerator>, hold: Hold, field: Field) {
    self.field = field;
    self.next_generator = gen;
    self.hold = hold;

    self.proceed_next();
  }

  pub fn turn_left(&mut self) -> bool {
    self.current.turn_left(&self.field)
  }

  pub fn turn_right(&mut self) -> bool {
    self.current.turn_right(&self.field)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::dev_utils::tests::tetsimu2::field::make_field;
  use crate::tetsimu2::core::Tetromino;
  use crate::tetsimu2::fixed_next_generator::FixedNextGenerator;

  #[test]
  fn hard_drop() {
    let gen = FixedNextGenerator::new(vec![Tetromino::I, Tetromino::J, Tetromino::L]);
    let mut conductor = FieldConductor::new(Box::new(gen));

    conductor.hard_drop();
    #[rustfmt::skip]
    assert_eq!(conductor.field, make_field("NNNIIIINNN"));

    conductor.hard_drop();
    #[rustfmt::skip]
    assert_eq!(conductor.field, make_field(
        &format!("{}{}{}",
            "NNNJNNNNNN",
            "NNNJJJNNNN",
            "NNNIIIINNN")));
  }

  #[test]
  fn hard_drop_should_be_dead_due_to_block_overlapped() {
    let gen = FixedNextGenerator::new(vec![Tetromino::T, Tetromino::I]);
    #[rustfmt::skip]
    let field = make_field(
      &format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
          "NNNNNNINNN", //______________ deadline
          "NNNNNNINNN",
          "NNNNNNNNNN",
          "NNNNNNNNNN",
          "NNNNNNNNNN",
          "NNNNNNNNNN",
          "NNNNNNNNNN",
          "NNNNNNNNNN",
          "NNNNNNNNNN",
          "NNNNNNNNNN",
          "NNNNNNNNNN",
          "NNNNNNNNNN",
          "NNNNNNNNNN",
          "NNNNNNNNNN",
          "NNNNNNNNNN",
          "NNNNNNNNNN",
          "NNNNNNNNNN",
          "NNNNNNNNNN",
          "NNNNNNNNNN",
          "NNNNNNNNNN",
          "NNNNNNNNNN"));
    let hold = Hold::new();
    let mut conductor = FieldConductor::new(Box::new(gen));
    let gen = FixedNextGenerator::new(vec![Tetromino::T, Tetromino::I]);
    conductor.reset(Box::new(gen), hold, field);

    conductor.hard_drop();
    assert_eq!(conductor.is_dead(), true);
  }

  #[test]
  fn hard_drop_should_be_dead_due_to_all_blocks_are_over_dead_line() {
    let gen = FixedNextGenerator::new(vec![Tetromino::T, Tetromino::I]);
    #[rustfmt::skip]
    let field = make_field(
      &format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
          "NNNNNNNNNN", //______________ deadline
          "NGGGGGGGGN",
          "NNNNNNNNNN",
          "NNNNNNNNNN",
          "NNNNNNNNNN",
          "NNNNNNNNNN",
          "NNNNNNNNNN",
          "NNNNNNNNNN",
          "NNNNNNNNNN",
          "NNNNNNNNNN",
          "NNNNNNNNNN",
          "NNNNNNNNNN",
          "NNNNNNNNNN",
          "NNNNNNNNNN",
          "NNNNNNNNNN",
          "NNNNNNNNNN",
          "NNNNNNNNNN",
          "NNNNNNNNNN",
          "NNNNNNNNNN",
          "NNNNNNNNNN",
          "NNNNNNNNNN"));
    let hold = Hold::new();
    let mut conductor = FieldConductor::new(Box::new(gen));
    let gen = FixedNextGenerator::new(vec![Tetromino::T, Tetromino::I]);
    conductor.reset(Box::new(gen), hold, field);

    conductor.move_left();
    conductor.move_left();
    conductor.move_left();
    conductor.hard_drop();
    assert_eq!(conductor.is_dead(), true);
  }

  #[test]
  fn hold() {
    let gen = FixedNextGenerator::new(vec![Tetromino::I, Tetromino::J]);
    let mut conductor = FieldConductor::new(Box::new(gen));
    assert_eq!(conductor.hold(), true);
    assert_eq!(conductor.current.r#type, Tetromino::J);
    assert_eq!(conductor.can_hold(), false);
    assert_eq!(conductor.holded(), Some(Tetromino::I));
  }
}
