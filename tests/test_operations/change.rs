use super::*;

#[test]
fn _0001() {
  let a = v(&["a", "b", "c", "d", "e"]);
  let b = v(&["x", "b", "c", "d", "e"]);
  let modifications = compare(&a, &b);
  assert_eq!(1, modifications.len());
  assert_eq!(Op::Insert, modifications[0].op);
  assert_eq!(1, modifications[0].line1);
  assert_eq!(1, modifications[0].line2);
  assert_eq!(
    n(r#"
 1   -a
   1 +x
"#),
    report(&a, &b, &modifications)
  )
}

#[test]
fn _0002() {
  let a: Vec<String> = ["a", "b", "c", "d", "e"].map(String::from).to_vec();
  let b: Vec<String> = ["a", "b", "x", "d", "e"].map(String::from).to_vec();
  let modifications = compare(&a, &b);
  assert_eq!(1, modifications.len());
  assert_eq!(Op::Insert, modifications[0].op);
  assert_eq!(3, modifications[0].line1); // 3 means: 3rd line in the old file
  assert_eq!(3, modifications[0].line2); // 3 means: 3rd line of the new file
  assert_eq!(
    n(r#"
 1 1  a
 2 2  b
 3   -c
   3 +x
"#),
    report(&a, &b, &modifications)
  )
}

#[ignore]
#[test]
fn _0003() {
  let a: Vec<String> = ["a", "b", "c", "d", "e"].map(String::from).to_vec();
  let b: Vec<String> = ["a", "b", "c", "d", "x"].map(String::from).to_vec();
  let modifications = compare(&a, &b);
  assert_eq!(2, modifications.len());
  let modification = &modifications[0];
  assert_eq!(Op::Delete, modification.op);
  assert_eq!(5, modification.line1);
  assert_eq!(4, modification.line2);
  let modification = &modifications[1];
  assert_eq!(Op::Insert, modification.op);
  assert_eq!(5, modification.line1);
  assert_eq!(5, modification.line2);
  assert_eq!(
    n(r#"
 1 1  a
 2 2  b
 3 3  c
 4 4  d
   3 +x
"#),
    report(&a, &b, &modifications)
  )
}

#[test]
fn _0004() {
  let a: Vec<String> = ["a", "b", "c", "d", "e"].map(String::from).to_vec();
  let b: Vec<String> = ["a", "b", "c", "x"].map(String::from).to_vec();
  let modifications = compare(&a, &b);
  assert_eq!(3, modifications.len());
  let modification = &modifications[0];
  assert_eq!(Op::Delete, modification.op);
  assert_eq!(4, modification.line1);
  assert_eq!(3, modification.line2);
  let modification = &modifications[1];
  assert_eq!(Op::Delete, modification.op);
  assert_eq!(5, modification.line1);
  assert_eq!(3, modification.line2);
  let modification = &modifications[2];
  assert_eq!(Op::Insert, modification.op);
  assert_eq!(5, modification.line1);
  assert_eq!(4, modification.line2);
}

#[ignore]
#[test]
fn _0005() {
  let a: Vec<String> = ["a", "b", "c", "d", "e"].map(String::from).to_vec();
  let b: Vec<String> = ["a", "b", "e", "x"].map(String::from).to_vec();
  let modifications = compare(&a, &b);
  assert_eq!(3, modifications.len());
  let modification = &modifications[0];
  assert_eq!(Op::Delete, modification.op);
  assert_eq!(3, modification.line1);
  assert_eq!(2, modification.line2);
  let modification = &modifications[1];
  assert_eq!(Op::Delete, modification.op);
  assert_eq!(4, modification.line1);
  assert_eq!(2, modification.line2);
  let modification = &modifications[2];
  assert_eq!(Op::Insert, modification.op);
  assert_eq!(5, modification.line1);
  assert_eq!(4, modification.line2);
}
