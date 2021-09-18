use crate::tetsimu2::core::MAX_FIELD_HEIGHT;
use crate::tetsimu2::current_tetromino::CurrentTetromino;
use crate::tetsimu2::field::Field;
use std::collections::HashSet;
use std::collections::VecDeque;

pub struct RouteSearcher<'a> {
  pub field: &'a Field,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum SearchRouteAction {
  MoveLeft = 1,
  MoveRight = 2,
  TurnLeft = 3,
  TurnRight = 4,
  SoftDrop = 5,
}

struct StackItem {
  start: CurrentTetromino,
  route_actions: Vec<SearchRouteAction>,
}

struct RouteSearchContext<'a> {
  queue: VecDeque<StackItem>,
  searched: HashSet<i32>,
  goal: &'a CurrentTetromino,
}

impl<'a> RouteSearcher<'a> {
  pub fn search_route(
    &self,
    start: CurrentTetromino,
    goal: &CurrentTetromino,
  ) -> Option<Vec<SearchRouteAction>> {
    let mut context = RouteSearchContext {
      queue: VecDeque::from(vec![StackItem {
        start,
        route_actions: vec![],
      }]),
      searched: HashSet::new(),
      goal,
    };

    while let Some(item) = context.queue.pop_front() {
      if let Some(route_actions) = self.search_with_hard_drop(&context, &item) {
        return Some(route_actions);
      }

      if let Some(route_actions) = self.search_with_turn_left(&mut context, &item) {
        return Some(route_actions);
      }

      if let Some(route_actions) = self.search_with_turn_right(&mut context, &item) {
        return Some(route_actions);
      }

      if let Some(route_actions) = self.search_with_move_left(&mut context, &item) {
        return Some(route_actions);
      }

      if let Some(route_actions) = self.search_with_move_right(&mut context, &item) {
        return Some(route_actions);
      }

      if let Some(route_actions) = self.search_with_soft_drop(&mut context, &item) {
        return Some(route_actions);
      }
    }

    None
  }

  fn search_with_hard_drop(
    &self,
    context: &RouteSearchContext,
    item: &StackItem,
  ) -> Option<Vec<SearchRouteAction>> {
    let mut current = item.start.clone();

    current.drop_to_bottom(&self.field);

    if current == *context.goal {
      Some(item.route_actions.clone())
    } else {
      None
    }
  }

  fn search_with_turn_left(
    &self,
    context: &mut RouteSearchContext,
    item: &StackItem,
  ) -> Option<Vec<SearchRouteAction>> {
    self.serch_with(
      context,
      item,
      |current| current.turn_left(&self.field),
      SearchRouteAction::TurnLeft,
    )
  }

  fn search_with_turn_right(
    &self,
    context: &mut RouteSearchContext,
    item: &StackItem,
  ) -> Option<Vec<SearchRouteAction>> {
    self.serch_with(
      context,
      item,
      |current| current.turn_right(&self.field),
      SearchRouteAction::TurnRight,
    )
  }

  fn search_with_move_left(
    &self,
    context: &mut RouteSearchContext,
    item: &StackItem,
  ) -> Option<Vec<SearchRouteAction>> {
    self.serch_with(
      context,
      item,
      |current| current.move_left(&self.field),
      SearchRouteAction::MoveLeft,
    )
  }

  fn search_with_move_right(
    &self,
    context: &mut RouteSearchContext,
    item: &StackItem,
  ) -> Option<Vec<SearchRouteAction>> {
    self.serch_with(
      context,
      item,
      |current| current.move_right(&self.field),
      SearchRouteAction::MoveRight,
    )
  }

  fn search_with_soft_drop(
    &self,
    context: &mut RouteSearchContext,
    item: &StackItem,
  ) -> Option<Vec<SearchRouteAction>> {
    self.serch_with(
      context,
      item,
      |current| current.soft_drop(&self.field),
      SearchRouteAction::SoftDrop,
    )
  }

  fn serch_with<F: FnOnce(&mut CurrentTetromino) -> bool>(
    &self,
    context: &mut RouteSearchContext,
    item: &StackItem,
    op: F,
    action: SearchRouteAction,
  ) -> Option<Vec<SearchRouteAction>> {
    let mut current = item.start.clone();

    if !op(&mut current) {
      return None;
    }

    let mut route_actions = item.route_actions.clone();
    route_actions.push(action);

    if current == *context.goal {
      Some(route_actions)
    } else {
      let value = self.calculate_current_value(&current);
      if context.searched.contains(&value) {
        return None;
      }

      context.queue.push_back(StackItem {
        start: current,
        route_actions,
      });

      context.searched.insert(value);
      None
    }
  }

  fn calculate_current_value(&self, current: &CurrentTetromino) -> i32 {
    let mut value = current.y * (MAX_FIELD_HEIGHT + 1) + current.x + 1;
    value <<= 2;
    value += current.direction as i32;
    value <<= 3;
    value += current.r#type as i32;

    value
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::dev_utils::tests::tetsimu2::field::make_field;
  use crate::tetsimu2::core::Direction;
  use crate::tetsimu2::core::Tetromino;

  #[test]
  fn search_route_should_return_null_if_route_is_not_found() {
    #[rustfmt::skip]
    let field = make_field(
      &format!("{}{}{}{}{}",
          "NNNNNNNNNN",
          "NNNNNNNNNN",
          "GGGGGGGGGG",
          "NNNNNNNNNN",
          "NNNNNNNNNN"));

    let start = CurrentTetromino {
      r#type: Tetromino::O,
      direction: Direction::Up,
      x: 4,
      y: 5,
    };

    let goal = CurrentTetromino {
      r#type: Tetromino::O,
      direction: Direction::Up,
      x: 4,
      y: 0,
    };

    let route_searcher = RouteSearcher { field: &field };
    assert_eq!(route_searcher.search_route(start, &goal), None);
  }

  #[test]
  fn search_route_should_return_routes_if_start_is_placed_at_just_harddrop_to_goal() {
    #[rustfmt::skip]
    let field = make_field(
      &format!("{}{}{}{}{}",
        "NNNNNNNNNN",
        "NNNNNNNNNN",
        "NNNNNNNNNN",
        "NNNNNNNNNN",
        "NNNNNNNNNN"));

    let start = CurrentTetromino {
      r#type: Tetromino::O,
      direction: Direction::Up,
      x: 4,
      y: 5,
    };

    let goal = CurrentTetromino {
      r#type: Tetromino::O,
      direction: Direction::Up,
      x: 4,
      y: 0,
    };

    let route_searcher = RouteSearcher { field: &field };
    assert_eq!(route_searcher.search_route(start, &goal), Some(vec![]));
  }

  #[test]
  fn search_route_should_return_routes_with_left() {
    #[rustfmt::skip]
    let field = make_field(
      &format!("{}{}{}{}{}",
        "NNNNNNNNNN",
        "NNGGGGGGGG",
        "NNNNNNNNNN",
        "NNNNNNNNNN",
        "NNNNNNNNNN"));

    let start = CurrentTetromino {
      r#type: Tetromino::O,
      direction: Direction::Up,
      x: 4,
      y: 5,
    };

    let goal = CurrentTetromino {
      r#type: Tetromino::O,
      direction: Direction::Up,
      x: 0,
      y: 0,
    };

    let route_searcher = RouteSearcher { field: &field };
    assert_eq!(
      route_searcher.search_route(start, &goal),
      Some(vec![
        SearchRouteAction::MoveLeft,
        SearchRouteAction::MoveLeft,
        SearchRouteAction::MoveLeft,
        SearchRouteAction::MoveLeft,
      ])
    );
  }

  #[test]
  fn search_route_should_return_routes_with_left_softdrop_right() {
    #[rustfmt::skip]
    let field = make_field(
      &format!("{}{}{}{}{}",
        "NNNNNNNNNN",
        "NNGGGGGGGG",
        "NNNNNNNNNN",
        "NNNNNNNNNN",
        "NNNNNNNNNN"));

    let start = CurrentTetromino {
      r#type: Tetromino::O,
      direction: Direction::Up,
      x: 4,
      y: 5,
    };

    let goal = CurrentTetromino {
      r#type: Tetromino::O,
      direction: Direction::Up,
      x: 4,
      y: 0,
    };

    let route_searcher = RouteSearcher { field: &field };
    assert_eq!(
      route_searcher.search_route(start, &goal),
      Some(vec![
        SearchRouteAction::MoveLeft,
        SearchRouteAction::MoveLeft,
        SearchRouteAction::MoveLeft,
        SearchRouteAction::MoveLeft,
        SearchRouteAction::SoftDrop,
        SearchRouteAction::SoftDrop,
        SearchRouteAction::SoftDrop,
        SearchRouteAction::SoftDrop,
        SearchRouteAction::MoveRight,
        SearchRouteAction::MoveRight,
        SearchRouteAction::MoveRight,
        SearchRouteAction::MoveRight,
      ])
    );
  }

  #[test]
  fn search_route_should_return_routes_with_left_and_right_srss() {
    #[rustfmt::skip]
    let field = make_field(
      &format!("{}{}{}{}{}{}{}{}{}",
        "GGGNNNGGGG",
        "GGGNNNNGGG",
        "GGGGGGNGGG",
        "GGGGGNNGGG",
        "GGGGGNNGGG",
        "GGGGNNNGGG",
        "GGGGNGGGGG",
        "GGGGNNGGGG",
        "GGGGNNGGGG"));

    let start = CurrentTetromino {
      r#type: Tetromino::T,
      direction: Direction::Up,
      x: 4,
      y: 7,
    };

    let goal = CurrentTetromino {
      r#type: Tetromino::T,
      direction: Direction::Right,
      x: 4,
      y: 1,
    };

    let route_searcher = RouteSearcher { field: &field };
    assert_eq!(
      route_searcher.search_route(start, &goal),
      Some(vec![
        SearchRouteAction::MoveRight,
        SearchRouteAction::TurnLeft,
        SearchRouteAction::SoftDrop,
        SearchRouteAction::TurnRight,
        SearchRouteAction::TurnRight,
      ])
    );
  }
}
