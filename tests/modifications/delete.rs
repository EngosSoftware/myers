use super::*;

#[test]
fn _0001a() {
  let a = v(&["a"]);
  let b = v(&[]);
  assert_eq!(
    n(r#"
 1   - a
"#),
    report(&a, &b, &compare(&a, &b), CM)
  );
}

#[test]
fn _0001() {
  let a = v(&["a", "b", "c", "d", "e"]);
  let b = v(&["b", "c", "d", "e"]);
  assert_eq!(
    n(r#"
 1   - a
 2 1   b
 3 2   c
 4 3   d
 5 4   e
"#),
    report(&a, &b, &compare(&a, &b), CM)
  );
}

#[test]
fn _0002() {
  let a = v(&["a", "b", "c", "d", "e"]);
  let b = v(&["a", "b", "d", "e"]);
  assert_eq!(
    n(r#"
 1 1   a
 2 2   b
 3   - c
 4 3   d
 5 4   e
"#),
    report(&a, &b, &compare(&a, &b), CM)
  );
}

#[test]
fn _0003() {
  let a = v(&["a", "b", "c", "d", "e"]);
  let b = v(&["a", "b", "c", "d"]);
  assert_eq!(
    n(r#"
 1 1   a
 2 2   b
 3 3   c
 4 4   d
 5   - e
"#),
    report(&a, &b, &compare(&a, &b), CM)
  );
}

#[test]
fn _0004() {
  let a = v(&["a", "b", "c"]);
  let b = v(&[]);
  assert_eq!(
    n(r#"
 1   - a
 2   - b
 3   - c
"#),
    report(&a, &b, &compare(&a, &b), CM)
  );
}

#[test]
fn _0005() {
  let a = v(&["a", "b", "c"]);
  let b = v(&["a", "c"]);
  assert_eq!(
    n(r#"
 1 1   a
 2   - b
 3 2   c
"#),
    report(&a, &b, &compare(&a, &b), CM)
  );
}

#[test]
fn _0006() {
  diff(
    false,
    "a,a,a,a",
    "a,a",
    r#"
 1 1   a
 2 2   a
 3   - a
 4   - a
"#,
    r#"
Delete 3 2
Delete 4 2
"#,
  );
}
