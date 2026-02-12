use super::*;

/// Compared files are empty.
#[test]
fn _0001() {
  let a = vec![];
  let b = vec![];
  assert_eq!(
    n(r#"
"#),
    report(&a, &b, &compare(&a, &b))
  )
}

/// Compared files have a single line.
#[test]
fn _0002() {
  let a = v(&["a"]);
  let b = v(&["a"]);
  assert_eq!(
    n(r#"
 1 1  a
"#),
    report(&a, &b, &compare(&a, &b))
  )
}

/// Compared files have multiple lines.
#[test]
fn _0003() {
  let a = v(&["a", "b", "c", "d", "e"]);
  let b = v(&["a", "b", "c", "d", "e"]);
  assert_eq!(
    n(r#"
 1 1  a
 2 2  b
 3 3  c
 4 4  d
 5 5  e
"#),
    report(&a, &b, &compare(&a, &b))
  )
}
