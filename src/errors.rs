//! # Definition of result and errors

use std::fmt;
use std::fmt::Display;

/// Common result type.
pub type Result<T, E = MyersError> = std::result::Result<T, E>;

/// Error definition.
#[derive(Debug, PartialEq, Eq)]
pub struct MyersError(String);

impl Display for MyersError {
  /// Implementation of [Display] trait for [MyersError].
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl MyersError {
  /// Creates a new [MyersError] with specified error message.
  pub fn new(message: impl AsRef<str>) -> Self {
    Self(message.as_ref().to_string())
  }
}

macro_rules! err {
  ($($arg:tt)*) => {{
    Err(MyersError::new(format!($($arg)*)))
  }};
}

pub(crate) use err;
