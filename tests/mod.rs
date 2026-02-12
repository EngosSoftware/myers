mod modifications;

use antex::ColorMode;
use myers::{Modification, compare, report};

const CM: ColorMode = ColorMode::Off;

/// Converts a slice of strings to vector of strings.
fn v(s: &[&str]) -> Vec<String> {
  s.iter().map(|s| s.to_string()).collect::<Vec<String>>()
}

/// Removes the first character from provided string.
fn n(s: &str) -> &str {
  &s[1..]
}

#[allow(dead_code)]
fn debug_modifications(modifications: &[Modification]) {
  for modification in modifications {
    println!("{:?}", modification);
  }
}

fn df(a: &str, b: &str, expected: &str) {
  let a = a.chars().map(|c| c.to_string()).collect::<Vec<String>>();
  let b = b.chars().map(|c| c.to_string()).collect::<Vec<String>>();
  assert_eq!(n(expected), report(&a, &b, &compare(&a, &b), CM));
}
