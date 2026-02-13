mod modifications;

use antex::ColorMode;
use myers::{Modification, compare, report};
use std::fmt::Write;

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

fn diff(trace: bool, a: &str, b: &str, expected_output: &str, expected_edits: &str) {
  let a = a.split(',').map(|s| s.to_string()).collect::<Vec<String>>();
  let b = b.split(',').map(|s| s.to_string()).collect::<Vec<String>>();
  let modifications = compare(&a, &b);
  if trace {
    for modification in &modifications {
      println!("{:?} {} {}", modification.op, modification.line_1, modification.line_2);
    }
  }
  let mut actual_edits = String::new();
  for modification in &modifications {
    _ = writeln!(&mut actual_edits, "{:?} {} {}", modification.op, modification.line_1, modification.line_2);
  }
  assert!(!expected_output.is_empty(), "expected output may not be empty");
  assert_eq!(expected_output[1..], report(&a, &b, &modifications, CM));
  assert_eq!(expected_edits.trim(), actual_edits.trim());
}
