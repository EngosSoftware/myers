mod cli;
mod errors;

use crate::cli::{CliAction, action};
use antex::{StyledText, Text, auto};
use myers::{compare, report};

/// Reads the input file.
fn read_file(file_name: &str) -> Vec<String> {
  let content = std::fs::read_to_string(file_name).unwrap_or_else(|e| {
    eprintln!(
      "{} Failed to read input file '{}'\n  {}",
      auto().bright_red().bold().s("error:").reset(),
      auto().yellow().s(file_name).reset(),
      e
    );
    std::process::exit(1);
  });
  let mut buffer = vec![];
  for line in content.lines() {
    buffer.push(line.to_string());
  }
  buffer
}

fn main() {
  match action() {
    Ok(action) => match action {
      CliAction::Compare(options, file1, file2) => {
        let file1 = read_file(&file1);
        let file2 = read_file(&file2);
        let modifications = compare(&file1, &file2);
        print!("{}", report(&file1, &file2, &modifications, options.cm()));
      }
      CliAction::Version => {
        println!("{}", auto().bold().bright_blue().s(cli::VERSION));
      }
      CliAction::Help => {
        print!("{}", cli::usage());
        println!("\n{}", auto().yellow().bold().s("help"));
      }
      CliAction::Usage => {
        print!("{}", cli::usage());
      }
    },
    Err(reason) => {
      eprintln!("{} {}", auto().bright_red().bold().s("error:").reset(), reason);
      eprintln!("\nFor more information, try '{}'.", Text::auto().bright_cyan().bold().s("--help").reset());
      std::process::exit(1);
    }
  }
}
