use myers::{Modification, compare};

/// Reads the input file.
fn read_file(file_name: &str) -> Vec<String> {
  let content = std::fs::read_to_string(file_name).unwrap_or_else(|e| {
    eprintln!("ERROR: Failed to read input file: {}\n{}", file_name, e);
    std::process::exit(1);
  });
  let mut buffer = vec![];
  for line in content.lines() {
    buffer.push(line.to_string());
  }
  buffer
}

/// Prints the edit script to standard output.
fn print_report(file1: &[String], file2: &[String], modifications: &[Modification]) {
  let (_, _) = (file1, file2);
  println!("WORK IN PROGRESS; the following report will be better ;-)");
  for modification in modifications {
    println!("{:?}", modification);
  }
}

fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() == 3 {
    let file1 = read_file(&args[1]);
    let file2 = read_file(&args[2]);
    let modifications = compare(&file1, &file2);
    print_report(&file1, &file2, &modifications);
  } else {
    eprintln!("WORK IN PROGRESS");
  }
}
