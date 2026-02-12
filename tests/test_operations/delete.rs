use super::*;

#[test]
fn _0001() {
  let a = v(&["a", "b", "c", "d", "e"]);
  let b = v(&["b", "c", "d", "e"]);
  assert_eq!(
    n(r#"
 1   -a
 2 1  b
 3 2  c
 4 3  d
 5 4  e
"#),
    report(&a, &b, &compare(&a, &b))
  );
}

#[test]
fn _0002() {
  let a = v(&["a", "b", "c", "d", "e"]);
  let b = v(&["a", "b", "d", "e"]);
  assert_eq!(
    n(r#"
 1 1  a
 2 2  b
 3   -c
 4 3  d
 5 4  e
"#),
    report(&a, &b, &compare(&a, &b))
  );
}

#[test]
fn _0003() {
  let a = v(&["a", "b", "c", "d", "e"]);
  let b = v(&["a", "b", "c", "d"]);
  let modifications = compare(&a, &b);
  assert_eq!(1, modifications.len());
  assert_eq!(Op::Delete, modifications[0].op);
  assert_eq!(5, modifications[0].line_1);
  assert_eq!(4, modifications[0].line_2);
  assert_eq!(
    n(r#"
 1 1  a
 2 2  b
 3 3  c
 4 4  d
 5   -e
"#),
    report(&a, &b, &compare(&a, &b))
  );
}
