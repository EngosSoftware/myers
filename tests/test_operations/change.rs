use super::*;

#[test]
fn _0001() {
  let a = v(&["a", "b", "c", "d", "e"]);
  let b = v(&["x", "b", "c", "d", "e"]);
  let modifications = compare(&a, &b);
  assert_eq!(1, modifications.len());
  assert_eq!(Op::Insert, modifications[0].op);
  assert_eq!(1, modifications[0].line_1);
  assert_eq!(1, modifications[0].line_2);
  assert_eq!(
    n(r#"
 1   -a
   1 +x
 2 2  b
 3 3  c
 4 4  d
 5 5  e
"#),
    report(&a, &b, &modifications)
  );
}

#[test]
fn _0002() {
  let a = v(&["a", "b", "c", "d", "e"]);
  let b = v(&["a", "b", "x", "d", "e"]);
  let modifications = compare(&a, &b);
  assert_eq!(1, modifications.len());
  assert_eq!(Op::Insert, modifications[0].op);
  assert_eq!(3, modifications[0].line_1); // 3 means: 3rd line in the old file
  assert_eq!(3, modifications[0].line_2); // 3 means: 3rd line of the new file
  assert_eq!(
    n(r#"
 1 1  a
 2 2  b
 3   -c
   3 +x
 4 4  d
 5 5  e
"#),
    report(&a, &b, &modifications)
  );
}

#[test]
fn _0003() {
  let a = v(&["a", "b", "c", "d", "e"]);
  let b = v(&["a", "b", "c", "d", "x"]);
  assert_eq!(
    n(r#"
 1 1  a
 2 2  b
 3 3  c
 4 4  d
 5   -e
   5 +x
"#),
    report(&a, &b, &compare(&a, &b))
  );
}

#[test]
fn _0004() {
  let a: Vec<String> = ["a", "b", "c", "d", "e"].map(String::from).to_vec();
  let b: Vec<String> = ["a", "b", "c", "x"].map(String::from).to_vec();
  assert_eq!(
    n(r#"
 1 1  a
 2 2  b
 3 3  c
 4   -d
 5   -e
   4 +x
"#),
    report(&a, &b, &compare(&a, &b))
  );
}

#[test]
fn _0005() {
  let a = v(&["a", "b", "c", "d", "e"]);
  let b = v(&["a", "b", "e", "x"]);
  assert_eq!(
    n(r#"
 1 1  a
 2 2  b
 3   -c
 4   -d
 5 3  e
   4 +x
"#),
    report(&a, &b, &compare(&a, &b))
  );
}
