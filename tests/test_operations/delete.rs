use super::*;

#[test]
fn single_delete_at_the_beginning() {
  let file1: Vec<String> = ["a", "b", "c", "d", "e"].map(String::from).to_vec();
  let file2: Vec<String> = ["b", "c", "d", "e"].map(String::from).to_vec();
  let modifications = compare(&file1, &file2);
  assert_eq!(1, modifications.len());
  assert_eq!(Op::Delete, modifications[0].op);
  assert_eq!(1, modifications[0].line1); // delete the 1st line in the old file to get the new one
  assert_eq!(0, modifications[0].line2);
}

#[test]
fn single_delete_in_the_middle() {
  let file1: Vec<String> = ["a", "b", "c", "d", "e"].map(String::from).to_vec();
  let file2: Vec<String> = ["a", "b", "d", "e"].map(String::from).to_vec();
  let modifications = compare(&file1, &file2);
  assert_eq!(1, modifications.len());
  assert_eq!(Op::Delete, modifications[0].op);
  assert_eq!(3, modifications[0].line1); // delete the 3rd line in the old file to get the new one
  assert_eq!(2, modifications[0].line2);
}

#[test]
fn single_delete_at_the_end() {
  let file1: Vec<String> = ["a", "b", "c", "d", "e"].map(String::from).to_vec();
  let file2: Vec<String> = ["a", "b", "c", "d"].map(String::from).to_vec();
  let modifications = compare(&file1, &file2);
  assert_eq!(1, modifications.len());
  assert_eq!(Op::Delete, modifications[0].op);
  assert_eq!(5, modifications[0].line1); // delete the 5th line in the old file to get the new one
  assert_eq!(4, modifications[0].line2);
}
