use crate::tetsimu2::core::Tetromino;

#[derive(Debug, Eq, PartialEq)]
pub enum HoldError {
  HoldTwice(String),
}

#[derive(Debug, Eq, PartialEq)]
pub struct Hold {
  holded: Option<Tetromino>,
  can_hold: bool,
}

impl Hold {
  pub fn new() -> Hold {
    Hold {
      holded: None,
      can_hold: true,
    }
  }

  pub fn can_hold(&self) -> bool {
    self.can_hold
  }

  pub fn holded(&self) -> Option<Tetromino> {
    self.holded
  }

  pub fn make_holdable(&mut self) {
    self.can_hold = true;
  }

  pub fn hold(&mut self, tetromino_to_hold: Tetromino) -> Result<Option<Tetromino>, HoldError> {
    if !self.can_hold() {
      Err(HoldError::HoldTwice(String::from(
        "Cannott hold consecutively",
      )))
    } else if self.holded.is_some() {
      self.can_hold = false;
      let tetromino_to_return = self.holded;
      self.holded = Some(tetromino_to_hold);
      Ok(tetromino_to_return)
    } else {
      self.can_hold = false;
      self.holded = Some(tetromino_to_hold);
      Ok(None)
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new() {
    let hold = Hold::new();
    assert_eq!(hold.can_hold(), true);
    assert_eq!(hold.holded(), None);
  }

  #[test]
  fn hold() {
    let mut hold = Hold::new();
    let hold_ret = hold.hold(Tetromino::I);

    assert_eq!(hold_ret, Ok(None));
    assert_eq!(hold.can_hold(), false);

    hold.make_holdable();
    assert_eq!(hold.can_hold(), true);

    let hold_ret = hold.hold(Tetromino::J);
    assert_eq!(hold_ret, Ok(Some(Tetromino::I)));
  }
}
