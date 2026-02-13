use std::fmt::Debug;
use std::rc::Rc;

/// Operation on the old file.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Op {
  /// Insert a line.
  Insert,
  /// Delete a line.
  Delete,
}

/// Modification details.
#[derive(Debug, Clone)]
pub struct Modification {
  /// Operation to be performed on the old file.
  pub op: Op,
  /// Line number in the old file.
  pub line_1: usize,
  /// Line number in the new file.
  pub line_2: usize,
}

impl Modification {
  /// Creates a new modification details.
  pub fn new(op: Op, line_1: usize, line_2: usize) -> Self {
    Self { op, line_1, line_2 }
  }
}

/// Edit details.
#[derive(Debug, Clone)]
pub struct Edit {
  /// Modification details.
  pub modification: Modification,
  /// Link to the previous operation.
  pub link: Option<Rc<Edit>>,
}

impl Edit {
  /// Creates a new edit details.
  pub fn new(modification: Modification, link: Option<Rc<Edit>>) -> Self {
    Self { modification, link }
  }
}
