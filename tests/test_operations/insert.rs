use myers::{Op, compare};

#[test]
fn single_insert_at_the_beginning() {
  let a: Vec<String> = ["b", "c", "d", "e"].map(String::from).to_vec();
  let b: Vec<String> = ["a", "b", "c", "d", "e"].map(String::from).to_vec();
  let modifications = compare(&a, &b);
  assert_eq!(1, modifications.len());
  assert_eq!(Op::Insert, modifications[0].op);
  assert_eq!(0, modifications[0].line_1);
  assert_eq!(1, modifications[0].line_2);
}

#[test]
fn single_insert_in_the_middle() {
  let a: Vec<String> = ["a", "b", "d", "e"].map(String::from).to_vec();
  let b: Vec<String> = ["a", "b", "c", "d", "e"].map(String::from).to_vec();
  let modifications = compare(&a, &b);
  assert_eq!(1, modifications.len());
  assert_eq!(Op::Insert, modifications[0].op);
  assert_eq!(2, modifications[0].line_1);
  assert_eq!(3, modifications[0].line_2);
}

#[test]
fn single_insert_at_the_end() {
  let a: Vec<String> = ["a", "b", "c", "d"].map(String::from).to_vec();
  let b: Vec<String> = ["a", "b", "c", "d", "e"].map(String::from).to_vec();
  let modifications = compare(&a, &b);
  assert_eq!(1, modifications.len());
  assert_eq!(Op::Insert, modifications[0].op);
  assert_eq!(4, modifications[0].line_1);
  assert_eq!(5, modifications[0].line_2);
}
