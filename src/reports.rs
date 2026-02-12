use crate::common::{Modification, Op};
use std::fmt::Write;

pub fn report(file_1: &[String], file_2: &[String], modifications: &[Modification]) -> String {
  let mut report = String::new();
  let width_1 = digits(file_1.len());
  let width_2 = digits(file_2.len());
  let mut last_line = 1;
  for m in modifications {
    let line = m.line1;
    while last_line < line {
      _ = writeln!(&mut report, " {0:>2$} {0:>3$}  {1:}", last_line, file_1[last_line - 1], width_1, width_2);
      last_line += 1;
    }
    match m.op {
      Op::Insert => {
        if m.line1 == m.line2 {
          _ = writeln!(&mut report, " {:>3$} {:>4$} -{}", m.line1, "", file_1[m.line1 - 1], width_1, width_2);
          _ = writeln!(&mut report, " {:>3$} {:>4$} +{}", "", m.line2, file_2[m.line2 - 1], width_1, width_2);
        }
      }
      Op::Delete => {}
    }
  }
  report
}

/// Returns the number of digits in the provided integer number.
pub fn digits(mut n: usize) -> usize {
  let mut count = 0;
  if n == 0 {
    count = 1;
  } else {
    while n > 0 {
      n /= 10;
      count += 1;
    }
  }
  count
}
