use super::*;

#[test]
fn _0001a() {
  let a = v(&["a"]);
  let b = v(&["b"]);
  assert_eq!(
    n(r#"
 1   - a
   1 + b
"#),
    report(&a, &b, &compare(&a, &b), CM)
  );
}

#[test]
fn _0001() {
  let a = v(&["a", "b", "c", "d", "e"]);
  let b = v(&["x", "b", "c", "d", "e"]);
  assert_eq!(
    n(r#"
 1   - a
   1 + x
 2 2   b
 3 3   c
 4 4   d
 5 5   e
"#),
    report(&a, &b, &compare(&a, &b), CM)
  );
}

#[test]
fn _0002a() {
  let a = v(&["a", "b", "c"]);
  let b = v(&["a", "x", "c"]);
  assert_eq!(
    n(r#"
 1 1   a
 2   - b
   2 + x
 3 3   c
"#),
    report(&a, &b, &compare(&a, &b), CM)
  );
}

#[test]
fn _0002() {
  let a = v(&["a", "b", "c", "d", "e"]);
  let b = v(&["a", "b", "x", "d", "e"]);
  assert_eq!(
    n(r#"
 1 1   a
 2 2   b
 3   - c
   3 + x
 4 4   d
 5 5   e
"#),
    report(&a, &b, &compare(&a, &b), CM)
  );
}

#[test]
fn _0003() {
  let a = v(&["a", "b", "c", "d", "e"]);
  let b = v(&["a", "b", "c", "d", "x"]);
  assert_eq!(
    n(r#"
 1 1   a
 2 2   b
 3 3   c
 4 4   d
 5   - e
   5 + x
"#),
    report(&a, &b, &compare(&a, &b), CM)
  );
}

#[test]
fn _0004() {
  let a = v(&["a", "b", "c", "d", "e"]);
  let b = v(&["a", "b", "c", "x"]);
  assert_eq!(
    n(r#"
 1 1   a
 2 2   b
 3 3   c
 4   - d
 5   - e
   4 + x
"#),
    report(&a, &b, &compare(&a, &b), CM)
  );
}

#[test]
fn _0005() {
  let a = v(&["a", "b", "c", "d", "e"]);
  let b = v(&["a", "b", "e", "x"]);
  assert_eq!(
    n(r#"
 1 1   a
 2 2   b
 3   - c
 4   - d
 5 3   e
   4 + x
"#),
    report(&a, &b, &compare(&a, &b), CM)
  );
}

#[test]
fn _0006() {
  let a = v(&["a", "b", "a", "b"]);
  let b = v(&["a", "a", "b", "b"]);
  assert_eq!(
    n(r#"
 1 1   a
 2   - b
 3 2   a
 4 3   b
   4 + b
"#),
    report(&a, &b, &compare(&a, &b), CM)
  );
}

#[test]
fn _0007() {
  df(
    "abcabba",
    "cbabac",
    r#"
 1 1   a
   2 + b
 3 3   c
 4 4   a
 5 5   b
 6   - b
   6 + c
"#,
  );

  let a = v(&["a", "b", "c", "a", "b", "b", "a"]);
  let b = v(&["c", "b", "a", "b", "a", "c"]);
  debug_modifications(&compare(&a, &b));
  assert_eq!(
    n(r#"
 1 1   a
   2 + b
 3 3   c
 4 4   a
 5 5   b
 6   - b
   6 + c
"#),
    report(&a, &b, &compare(&a, &b), CM)
  );
}
