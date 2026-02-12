use crate::common::{Edit, Modification, Op};

pub fn compare(a: &[String], b: &[String]) -> Vec<Modification> {
  let m = a.len();
  let n = b.len();
  let max_lines = if m >= n { m } else { n };
  let mut row = 0;
  while row < m && row < n && a[row] == b[row] {
    row += 1;
  }
  let mut last_d: Vec<usize> = vec![0; 2 * max_lines + 1];
  let mut script: Vec<Option<Box<Edit>>> = vec![None; 2 * max_lines + 1];
  last_d[max_lines] = row;
  script[max_lines] = None;
  let mut lower = if row == m { max_lines.saturating_add(1) } else { max_lines.saturating_sub(1) };
  let mut upper = if row == n { max_lines.saturating_sub(1) } else { max_lines.saturating_add(1) };
  let mut modifications = vec![];
  if lower <= upper {
    for d in 1..=(2 * max_lines) {
      for k in (lower..=upper).step_by(2) {
        let (op, link) = if k + d == max_lines || k != max_lines + d && last_d[k + 1] >= last_d[k - 1] {
          row = last_d[k + 1] + 1;
          (Op::Delete, script[k + 1].take())
        } else {
          row = last_d[k - 1];
          (Op::Insert, script[k - 1].take())
        };
        let mut col = row + k - max_lines;
        script[k] = Some(Box::new(Edit::new(Modification::new(op, row, col), link)));
        while row < m && col < n && a[row] == b[col] {
          row += 1;
          col += 1;
        }
        last_d[k] = row;
        if row == m && col == n {
          // Hit southeast corner - have the answer.
          let mut next = &script[k];
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
