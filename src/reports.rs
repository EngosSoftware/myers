use crate::common::{Modification, Op};
use antex::{ColorMode, StyledText, Text};
use std::fmt::Write;

const NOP: char = ' ';
const INS: char = '+';
const DEL: char = '-';

pub fn report(file_1: &[String], file_2: &[String], modifications: &[Modification], cm: ColorMode) -> String {
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
    println!("{:?}", m);
    match m.op {
      Op::Insert => {
        if m.line_1 == m.line_2 {
          while last_index_1 + 1 < m.line_1 {
            write(&mut report, NOP, last_index_1 + 1, last_index_2 + 1, &file_1[last_index_1], col_1, col_2, cm);
            last_index_1 += 1;
            last_index_2 += 1;
          }
          write(&mut report, DEL, m.line_1, 0, &file_1[m.line_1 - 1], col_1, col_2, cm);
          write(&mut report, INS, 0, m.line_2, &file_2[m.line_2 - 1], col_1, col_2, cm);
          last_index_1 += 1;
          last_index_2 += 1;
        } else if m.line_1 < m.line_2 {
          while last_index_1 + 1 < m.line_2 {
            write(&mut report, NOP, last_index_1 + 1, last_index_2 + 1, &file_1[last_index_1], col_1, col_2, cm);
            last_index_1 += 1;
            last_index_2 += 1;
          }
          write(&mut report, INS, 0, m.line_2, &file_2[m.line_2 - 1], col_1, col_2, cm);
          last_index_2 += 1;
        } else {
          while last_index_2 + 1 < m.line_2 {
            write(&mut report, NOP, last_index_1 + 1, last_index_2 + 1, &file_1[last_index_1], col_1, col_2, cm);
            last_index_1 += 1;
            last_index_2 += 1;
          }
          write(&mut report, INS, 0, m.line_2, &file_2[m.line_2 - 1], col_1, col_2, cm);
          last_index_1 += 1;
          last_index_2 += 1;
        }
      }
      Op::Delete => {
        while last_index_1 + 1 < m.line_1 {
          write(&mut report, NOP, last_index_1 + 1, last_index_2 + 1, &file_1[last_index_1], col_1, col_2, cm);
          last_index_1 += 1;
          last_index_2 += 1;
        }
        write(&mut report, DEL, m.line_1, 0, &file_1[m.line_1 - 1], col_1, col_2, cm);
        last_index_1 += 1;
        if let Some(m_after) = modifications.peek()
          && m_after.op == Op::Insert
          && m_after.line_1 == m.line_1
        {
          write(&mut report, INS, 0, m_after.line_2, &file_2[m_after.line_2 - 1], col_1, col_2, cm);
          last_index_2 += 1;
          _ = modifications.next();
        }
      }
    }
    modification = modifications.next();
  }
  while last_index_1 < len_1 && last_index_2 < len_2 {
    write(&mut report, NOP, last_index_1 + 1, last_index_2 + 1, &file_1[last_index_1], col_1, col_2, cm);
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

#[allow(clippy::too_many_arguments)]
fn write(s: &mut String, ch: char, n1: usize, n2: usize, l: &str, c1: usize, c2: usize, cm: ColorMode) {
  let n1 = if n1 > 0 { format!("{}", n1) } else { "".to_string() };
  let n2 = if n2 > 0 { format!("{}", n2) } else { "".to_string() };
  let text = format!(" {:>4$} {:>5$} {} {}", n1, n2, ch, l, c1, c2);
  println!("{}", text);
  _ = writeln!(
    s,
    "{}",
    match ch {
      INS => Text::new(cm).green().s(text),
      DEL => Text::new(cm).red().s(text),
      _ => Text::new(cm).s(text),
    }
  );
}
