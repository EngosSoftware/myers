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
  diff(
    false,
    "a,b,c,a,b,b,a",
    "c,b,a,b,a,c",
    r#"
 1   - a
 2   - b
 3 1   c
   2 + b
 4 3   a
 5 4   b
 6   - b
 7 5   a
   6 + c
"#,
    r#"
Delete 1 0
Delete 2 0
Insert 3 2
Delete 6 4
Insert 7 6
"#,
  );
}

#[test]
fn _0008() {
  diff(
    false,
    "a,b,c",
    "x,y,z",
    r#"
 1   - a
 2   - b
 3   - c
   1 + x
   2 + y
   3 + z
"#,
    r#"
Delete 1 0
Delete 2 0
Delete 3 0
Insert 3 1
Insert 3 2
Insert 3 3
"#,
  );
}

#[test]
fn _0009() {
  diff(
    false,
    "a,b,c,d,e,f",
    "a,b,c,x,y,z",
    r#"
 1 1   a
 2 2   b
 3 3   c
 4   - d
 5   - e
 6   - f
   4 + x
   5 + y
   6 + z
"#,
    r#"
Delete 4 3
Delete 5 3
Delete 6 3
Insert 6 4
Insert 6 5
Insert 6 6
"#,
  );
}

#[test]
fn _0010() {
  diff(
    false,
    "a,x,b,y,c,z",
    "a,b,c,z",
    r#"
 1 1   a
 2   - x
 3 2   b
 4   - y
 5 3   c
 6 4   z
"#,
    r#"
Delete 2 1
Delete 4 2
"#,
  );
}
