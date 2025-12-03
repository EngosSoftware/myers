use myers::compare;

#[test]
fn both_files_are_empty() {
  let a: Vec<String> = vec![];
  let b: Vec<String> = vec![];
  let modifications = compare(&a, &b);
  assert_eq!(0, modifications.len());
}

#[test]
fn both_files_have_one_line() {
  let a: Vec<String> = ["a"].map(String::from).to_vec();
  let b: Vec<String> = ["a"].map(String::from).to_vec();
  let modifications = compare(&a, &b);
  assert_eq!(0, modifications.len());
}

#[test]
fn both_files_have_multiple_lines() {
  let a: Vec<String> = ["a", "b", "c", "d", "e"].map(String::from).to_vec();
  let b: Vec<String> = ["a", "b", "c", "d", "e"].map(String::from).to_vec();
  let modifications = compare(&a, &b);
  assert_eq!(0, modifications.len());
}
