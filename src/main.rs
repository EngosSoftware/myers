use myers::{compare, report};

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

fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() == 3 {
    let file1 = read_file(&args[1]);
    let file2 = read_file(&args[2]);
    let modifications = compare(&file1, &file2);
    print!("{}", report(&file1, &file2, &modifications));
  } else {
    eprintln!("WORK IN PROGRESS");
  }
}
