use myers::{Modification, compare};

fn read_file(file_name: &str) -> Vec<String> {
  let content = std::fs::read_to_string(file_name).expect("failed to read input file");
  let mut buffer = vec![];
  for line in content.lines() {
    buffer.push(line.to_string());
  }
  buffer
}

fn print_report(file1: &[String], file2: &[String], modifications: &[Modification]) {
  let _ = file1;
  for modification in modifications {
    println!("+{} {:?}", file2[modification.line2 - 1], modification);
  }
}

fn main() {
  let file1 = read_file("file1.txt");
  let file2 = read_file("file2.txt");
  let Some(modifications) = compare(&file1, &file2) else {
    println!("Something went wrong.");
    return;
  };
  print_report(&file1, &file2, &modifications);
}
