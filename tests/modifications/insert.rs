use super::*;

#[test]
fn _0001a() {
  let a = v(&[]);
  let b = v(&["a"]);
  assert_eq!(
    n(r#"
   1 + a
"#),
    report(&a, &b, &compare(&a, &b), CM)
  );
}

#[test]
fn _0001() {
  let a = v(&["b", "c", "d", "e"]);
  let b = v(&["a", "b", "c", "d", "e"]);
  assert_eq!(
    n(r#"
   1 + a
 1 2   b
 2 3   c
 3 4   d
 4 5   e
"#),
    report(&a, &b, &compare(&a, &b), CM)
  );
}

#[test]
fn _0002() {
  let a = v(&["a", "b", "d", "e"]);
  let b = v(&["a", "b", "c", "d", "e"]);
  assert_eq!(
    n(r#"
 1 1   a
 2 2   b
   3 + c
 3 4   d
 4 5   e
"#),
    report(&a, &b, &compare(&a, &b), CM)
  );
}

#[test]
fn _0003() {
  let a = v(&["a", "b", "c", "d"]);
  let b = v(&["a", "b", "c", "d", "e"]);
  assert_eq!(
    n(r#"
 1 1   a
 2 2   b
 3 3   c
 4 4   d
   5 + e
"#),
    report(&a, &b, &compare(&a, &b), CM)
  );
}
