#[test]
fn no_arguments_should_display_usage_info() {
  cli_assert::command!().code(0).stdout(r#"aaa"#).stderr("").execute();
}
