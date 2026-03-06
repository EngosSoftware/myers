use crate::errors::{MyersError, Result, err};
use antex::{ColorMode, StyledText, Text, auto};
use std::iter::Peekable;

pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

const O_VERSION_SHORT: &str = "-V";
const O_VERSION_LONG: &str = "--version";
const O_COLOR_LONG: &str = "--color";
const O_COLOR_WHEN: &str = "<WHEN>";
const O_HELP_SHORT: &str = "-h";
const O_HELP_LONG: &str = "--help";

#[derive(Default)]
pub struct CliOptions {
  color_mode: Option<ColorMode>,
}

impl CliOptions {
  pub fn cm(&self) -> ColorMode {
    self.color_mode.unwrap_or_default()
  }
}

pub enum CliAction {
  Compare(CliOptions, String, String),
  Version,
  Help,
  Usage,
}

enum CliState {
  Start,
  Help,
  Version,
  Compare(String, String),
}

pub fn action() -> Result<CliAction> {
  let args = std::env::args().collect::<Vec<String>>();
  let mut args = args.iter().skip(1).peekable();
  let mut state = CliState::Start;
  let mut arg = args.next();
  let mut options: CliOptions = CliOptions::default();

  while let Some(value) = arg {
    match state {
      CliState::Start => match value.as_str() {
        O_VERSION_SHORT | O_VERSION_LONG => state = CliState::Version,
        O_HELP_SHORT | O_HELP_LONG => state = CliState::Help,
        v if v.starts_with(O_COLOR_LONG) => consume_color_mode(&mut options, v, &mut args)?,
        other if is_option(other) => return err!("{}", unexpected_option(other)),
        other => {
          let file1 = other.to_string();
          let Some(other) = args.next() else { return err!("missing second file") };
          let file2 = other.to_string();
          state = CliState::Compare(file1, file2);
        }
      },
      CliState::Compare(_, _) => {
        return match value.as_str() {
          other if is_option(other) => err!("{}", unexpected_option(other)),
          other => err!("{}", unexpected_argument(other)),
        };
      }
      CliState::Version => match value.as_str() {
        v @ O_VERSION_SHORT | v @ O_VERSION_LONG => return err!("{}", duplicated_option(v)),
        v @ O_HELP_SHORT | v @ O_HELP_LONG => return err!("conflicting option: {}", v),
        v if v.starts_with(O_COLOR_LONG) => consume_color_mode(&mut options, v, &mut args)?,
        other => return err!("{}", unexpected_argument(other)),
      },
      CliState::Help => match value.as_str() {
        v @ O_VERSION_SHORT | v @ O_VERSION_LONG => return err!("conflicting option: {}", v),
        v @ O_HELP_SHORT | v @ O_HELP_LONG => return err!("{}", duplicated_option(v)),
        v if v.starts_with(O_COLOR_LONG) => consume_color_mode(&mut options, v, &mut args)?,
        other if is_option(other) => return err!("{}", unexpected_option(other)),
        other => return err!("{}", unexpected_argument(other)),
      },
    }
    arg = args.next();
  }
  Ok(match state {
    CliState::Compare(file1, file2) => CliAction::Compare(options, file1, file2),
    CliState::Version => CliAction::Version,
    CliState::Help => CliAction::Help,
    _ => CliAction::Usage,
  })
}

fn consume_color_mode<'a, I>(options: &mut CliOptions, option: &str, args: &mut Peekable<I>) -> Result<()>
where
  I: Iterator<Item = &'a String>,
{
  if options.color_mode.is_some() {
    return err!("{}", duplicated_color_mode());
  } else if option.starts_with(&format!("{}=", O_COLOR_LONG)) {
    consume_color_mode_value(options, &option[O_COLOR_LONG.len() + 1..])?;
  } else if let Some(mode) = args.next() {
    consume_color_mode_value(options, mode)?;
  } else {
    return err!("{}", value_required_color_mode());
  }
  Ok(())
}

fn consume_color_mode_value(options: &mut CliOptions, mode: &str) -> Result<()> {
  if matches!(mode, "auto" | "always" | "never") {
    options.color_mode = ColorMode::new(mode).into();
  } else {
    return err!("{}", invalid_value_color_mode(mode));
  }
  Ok(())
}

pub fn usage() -> Text {
  const NL: &str = "\n";
  let comma_join = auto().s(", ");
  let space_join = auto().s("  ");
  let short_width = 4;
  // Program description.
  let description = auto().bright_white().s(DESCRIPTION).reset();
  // Usage message.
  let usage_label = auto().bold().bright_green().s("Usage: ").reset();
  let name = auto().bold().bright_cyan().s(NAME).reset();
  let args = auto().cyan().s(" [OPTIONS] <FILE_1> <FILE_2>").reset();
  let usage = usage_label + name + args;
  // List of options.
  let options_label = auto().bold().bright_green().s("Options:").reset();
  // 'version' option
  let version_short = auto().bold().bright_cyan().align_right(O_VERSION_SHORT, short_width).reset();
  let version_long = auto().bold().bright_cyan().s(O_VERSION_LONG).reset();
  let version_description = auto().indent(7, "Print version info and exit");
  let version = version_short + &comma_join + version_long + version_description;
  // 'color' option
  let color_short = auto().align_right("", short_width);
  let color_long = auto().bold().bright_cyan().s(O_COLOR_LONG).r(" ").cyan().s(O_COLOR_WHEN).reset();
  let color_description = auto().indent(2, "Coloring ") + color_mode_values();
  let color = color_short + &space_join + color_long + color_description;
  // 'help' option
  let help_short = auto().bold().bright_cyan().align_right(O_HELP_SHORT, short_width).reset();
  let help_long = auto().bold().bright_cyan().s(O_HELP_LONG).reset();
  let help_text = auto().indent(10, "Print help");
  let help = help_short + &comma_join + help_long + help_text;
  // final list of options
  let options = options_label + NL + version + NL + color + NL + help;
  // List of arguments.
  // TODO list of arguments
  description + NL + NL + usage + NL + NL + options + NL
}

fn unexpected_option(option: &str) -> Text {
  auto().s("unexpected option '").yellow().bold().s(option).r("' found")
}

fn unexpected_argument(argument: &str) -> String {
  format!("unexpected argument '{}' found", auto().yellow().bold().s(argument).reset())
}

fn duplicated_option(option: &str) -> String {
  format!("option '{}' can not be used multiple times", auto().yellow().bold().s(option).reset())
}

fn duplicated_color_mode() -> Text {
  auto()
    .s("option '")
    .bright_cyan()
    .bold()
    .s(O_COLOR_LONG)
    .r(" ")
    .cyan()
    .s(O_COLOR_WHEN)
    .r("' can not be used multiple times")
}

fn invalid_value_color_mode(value: &str) -> String {
  format!(
    "invalid value '{}' for '{}'\n  {}",
    auto().yellow().bold().s(value).reset(),
    auto().bright_cyan().bold().s(O_COLOR_LONG).r(" ").cyan().s(O_COLOR_WHEN).reset(),
    color_mode_values(),
  )
}

fn value_required_color_mode() -> Text {
  auto()
    .s("a value is required for '")
    .bright_cyan()
    .bold()
    .s(O_COLOR_LONG)
    .reset()
    .s(" ")
    .cyan()
    .s(O_COLOR_WHEN)
    .r("' but none was supplied\n  ")
    + color_mode_values()
}

fn color_mode_values() -> Text {
  auto()
    .s("[possible values: ")
    .cyan()
    .s("auto")
    .reset()
    .s(", ")
    .cyan()
    .s("always")
    .reset()
    .s(", ")
    .cyan()
    .s("never")
    .r("]")
}

fn is_option(option: &str) -> bool {
  option.starts_with("-")
}
