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
  pub line1: usize,
  /// Line number in the new file.
  pub line2: usize,
}

impl Modification {
  /// Creates a new modification details.
  fn new(op: Op, line1: usize, line2: usize) -> Self {
    Self { op, line1, line2 }
  }
}

/// Edit details.
#[derive(Debug, Clone)]
struct Edit {
  /// Modification details.
  modification: Modification,
  /// Link to the previous operation.
  link: Option<Box<Edit>>,
}

impl Edit {
  /// Creates a new edit details.
  fn new(modification: Modification, link: Option<Box<Edit>>) -> Self {
    Self { modification, link }
  }
}

pub fn compare(a: &[String], b: &[String]) -> Option<Vec<Modification>> {
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
  let mut lower = if row == m {
    max_lines.saturating_add(1)
  } else {
    max_lines.saturating_sub(1)
  };
  let mut upper = if row == n {
    max_lines.saturating_sub(1)
  } else {
    max_lines.saturating_add(1)
  };
  if lower > upper {
    // Compared files are identical, return an empty list of modifications.
    return Some(vec![]);
  }
  for d in 1..=(2 * max_lines) {
    for k in (lower..=upper).step_by(2) {
      let (op, link) = if k == max_lines - d || k != max_lines + d && last_d[k + 1] >= last_d[k - 1] {
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
        let mut modifications = vec![];
        let mut next = &script[k];
        while let Some(edit) = next {
          modifications.push(edit.modification.clone());
          next = &edit.link;
        }
        // Files are different, return and ordered list of modifications.
        modifications.reverse();
        return Some(modifications);
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
  None // Something went wrong.
}
