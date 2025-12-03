use myers::{Op, compare};

#[test]
fn single_insert_at_the_beginning() {
  let a: Vec<String> = ["b", "c", "d", "e"].map(String::from).to_vec();
  let b: Vec<String> = ["a", "b", "c", "d", "e"].map(String::from).to_vec();
  let modifications = compare(&a, &b);
  assert_eq!(1, modifications.len());
  assert_eq!(Op::Insert, modifications[0].op);
  assert_eq!(0, modifications[0].line1); // 0 means: before the 1st line in the old file
  assert_eq!(1, modifications[0].line2); // 1 means: 1st line of the new file
}

#[test]
fn single_insert_in_the_middle() {
  let a: Vec<String> = ["a", "b", "d", "e"].map(String::from).to_vec();
  let b: Vec<String> = ["a", "b", "c", "d", "e"].map(String::from).to_vec();
  let modifications = compare(&a, &b);
  assert_eq!(1, modifications.len());
  assert_eq!(Op::Insert, modifications[0].op);
  assert_eq!(2, modifications[0].line1); // 2 means: after the 2nd line in the old file
  assert_eq!(3, modifications[0].line2); // 3 means: 3rd line of the new file
}

#[test]
fn single_insert_at_the_end() {
  let a: Vec<String> = ["a", "b", "c", "d"].map(String::from).to_vec();
  let b: Vec<String> = ["a", "b", "c", "d", "e"].map(String::from).to_vec();
  let modifications = compare(&a, &b);
  assert_eq!(1, modifications.len());
  assert_eq!(Op::Insert, modifications[0].op);
  assert_eq!(4, modifications[0].line1); // 4 means: after the 4th line in the old file
  assert_eq!(5, modifications[0].line2); // 5 means: 5th line of the new file
}
