use crate::common::{Modification, Op};
use std::fmt::Write;

pub fn report(file_1: &[String], file_2: &[String], modifications: &[Modification]) -> String {
  let mut report = String::new();
  let len_1 = file_1.len();
  let len_2 = file_2.len();
  let col_1 = digits(len_1);
  let col_2 = digits(len_2);
  let mut last_index_1 = 0;
  let mut last_index_2 = 0;
  let mut modifications = modifications.iter().peekable();
  let mut modification = modifications.next();
  while modification.is_some() {
    let m = modification.unwrap();
    let line = m.line1;
    while last_index_1 + 1 < line {
      _ = writeln!(&mut report, " {:>3$} {:>4$}  {}", last_index_1 + 1, last_index_2 + 1, file_1[last_index_1], col_1, col_2);
      last_index_1 += 1;
      last_index_2 += 1;
    }
    match m.op {
      Op::Insert => {
        if m.line1 == m.line2 {
          _ = writeln!(&mut report, " {:>3$} {:>4$} -{}", m.line1, "", file_1[m.line1 - 1], col_1, col_2);
          _ = writeln!(&mut report, " {:>3$} {:>4$} +{}", "", m.line2, file_2[m.line2 - 1], col_1, col_2);
          last_index_1 += 1;
          last_index_2 += 1;
        }
      }
      Op::Delete => {}
    }
    modification = modifications.next();
  }
  while last_index_1 < len_1 && last_index_2 < len_2 {
    _ = writeln!(&mut report, " {:>3$} {:>4$}  {}", last_index_1 + 1, last_index_2 + 1, file_1[last_index_1], col_1, col_2);
    last_index_1 += 1;
    last_index_2 += 1;
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
