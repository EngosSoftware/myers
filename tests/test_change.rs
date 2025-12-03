use myers::{Op, compare};

#[test]
fn single_change_at_the_beginning() {
  let a: Vec<String> = ["a", "b", "c", "d", "e"].map(String::from).to_vec();
  let b: Vec<String> = ["x", "b", "c", "d", "e"].map(String::from).to_vec();
  let modifications = compare(&a, &b).unwrap();
  assert_eq!(1, modifications.len());
  assert_eq!(Op::Insert, modifications[0].op);
  assert_eq!(1, modifications[0].line1); // 1 means: 1st line in the old file
  assert_eq!(1, modifications[0].line2); // 1 means: 1st line of the new file
}

#[test]
fn single_change_in_the_middle() {
  let a: Vec<String> = ["a", "b", "c", "d", "e"].map(String::from).to_vec();
  let b: Vec<String> = ["a", "b", "x", "d", "e"].map(String::from).to_vec();
  let modifications = compare(&a, &b).unwrap();
  assert_eq!(1, modifications.len());
  assert_eq!(Op::Insert, modifications[0].op);
  assert_eq!(3, modifications[0].line1); // 3 means: 3rd line in the old file
  assert_eq!(3, modifications[0].line2); // 3 means: 3rd line of the new file
}

/// This case is strange.
#[test]
fn single_change_at_the_end() {
  let a: Vec<String> = ["a", "b", "c", "d", "e"].map(String::from).to_vec();
  let b: Vec<String> = ["a", "b", "c", "d", "x"].map(String::from).to_vec();
  let modifications = compare(&a, &b).unwrap();
  assert_eq!(2, modifications.len());
  let modification = &modifications[0];
  assert_eq!(Op::Delete, modification.op);
  assert_eq!(5, modification.line1);
  assert_eq!(4, modification.line2);
  let modification = &modifications[1];
  assert_eq!(Op::Insert, modification.op);
  assert_eq!(5, modification.line1);
  assert_eq!(5, modification.line2);
}
