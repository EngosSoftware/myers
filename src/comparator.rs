use crate::common::{Edit, Modification, Op};
use std::rc::Rc;

pub fn compare(a: &[String], b: &[String]) -> Vec<Modification> {
  let m = a.len() as isize;
  let n = b.len() as isize;
  let max_lines = if m >= n { m } else { n };
  let mut row = 0;
  while row < m && row < n && a[row as usize] == b[row as usize] {
    row += 1;
  }
  let mut last_d: Vec<usize> = vec![0; (2 * max_lines + 1) as usize];
  let mut script: Vec<Option<Rc<Edit>>> = vec![None; (2 * max_lines + 1) as usize];
  last_d[max_lines as usize] = row as usize;
  script[max_lines as usize] = None;
  let mut lower = if row == m { max_lines + 1 } else { max_lines - 1 };
  let mut upper = if row == n { max_lines - 1 } else { max_lines + 1 };
  let mut modifications = vec![];
  if lower <= upper {
    for d in 1..=(2 * max_lines) {
      for k in (lower..=upper).step_by(2) {
        let (op, link) = if k + d == max_lines || k != max_lines + d && last_d[(k + 1) as usize] >= last_d[(k - 1) as usize] {
          row = (last_d[(k + 1) as usize] + 1) as isize;
          (Op::Delete, script[(k + 1) as usize].as_ref().map(Rc::clone))
        } else {
          row = last_d[(k - 1) as usize] as isize;
          (Op::Insert, script[(k - 1) as usize].as_ref().map(Rc::clone))
        };
        let mut col = row + k - max_lines;
        script[k as usize] = Some(Rc::new(Edit::new(Modification::new(op, row as usize, col as usize), link)));
        while row < m && col < n && a[row as usize] == b[col as usize] {
          row += 1;
          col += 1;
        }
        last_d[k as usize] = row as usize;
        if row == m && col == n {
          // Hit southeast corner - have the answer.
          let mut next = &script[k as usize];
          while let Some(edit) = next {
            modifications.push(edit.modification.clone());
            next = &edit.link;
          }
          // Files are different, return and ordered list of modifications.
          modifications.reverse();
          return modifications;
        }
        if row == m {
          lower = k + 2;
        }
        if col == n {
          upper = k - 2;
        }
      }
      lower -= 1;
      upper += 1;
    }
  }
  modifications
}
