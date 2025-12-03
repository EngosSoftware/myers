use myers::{Op, compare};

#[test]
fn single_delete_at_the_beginning() {
  let a: Vec<String> = ["a", "b", "c", "d", "e"].map(String::from).to_vec();
  let b: Vec<String> = ["b", "c", "d", "e"].map(String::from).to_vec();
  let modifications = compare(&a, &b).unwrap();
  assert_eq!(1, modifications.len());
  assert_eq!(Op::Delete, modifications[0].op);
  assert_eq!(1, modifications[0].line1); // 1 means: delete 1st line in the old file
  assert_eq!(0, modifications[0].line2); // 0 means: no line before the first line in the new file
}

#[test]
fn single_delete_in_the_middle() {
  let a: Vec<String> = ["a", "b", "c", "d", "e"].map(String::from).to_vec();
  let b: Vec<String> = ["a", "b", "d", "e"].map(String::from).to_vec();
  let modifications = compare(&a, &b).unwrap();
  assert_eq!(1, modifications.len());
  assert_eq!(Op::Delete, modifications[0].op);
  assert_eq!(3, modifications[0].line1); // 3 means: delete 3rd line in the old file
  assert_eq!(2, modifications[0].line2); // 3 means: no line after 2nd line in the new file
}

#[test]
fn single_delete_at_the_end() {
  let a: Vec<String> = ["a", "b", "c", "d", "e"].map(String::from).to_vec();
  let b: Vec<String> = ["a", "b", "c", "d"].map(String::from).to_vec();
  let modifications = compare(&a, &b).unwrap();
  assert_eq!(1, modifications.len());
  assert_eq!(Op::Delete, modifications[0].op);
  assert_eq!(5, modifications[0].line1); // 5 means: delete 5th line in the old file
  assert_eq!(4, modifications[0].line2); // 4 means: no line after 4th line in the new file
}
