mod test_operations;

use myers::{Op, compare, report};

/// Converts a slice of strings to vector of strings.
fn v(s: &[&str]) -> Vec<String> {
  s.iter().map(|s| s.to_string()).collect::<Vec<String>>()
}

/// Removes the first character from provided string.
fn n(s: &str) -> &str {
  &s[1..]
}
