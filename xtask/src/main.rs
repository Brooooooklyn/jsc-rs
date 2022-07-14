use clap::Command;

mod build;

use build::build;

fn cli() -> Command<'static> {
  Command::new("xtask")
    .about("turbo-tooling cargo tasks")
    .subcommand_required(true)
    .arg_required_else_help(true)
    .allow_external_subcommands(true)
    .allow_invalid_utf8_for_external_subcommands(true)
    .subcommand(
      Command::new("build")
        .about("Build the JavaScriptCore and dependencies")
        .arg_required_else_help(false),
    )
}

fn main() {
  let matches = cli().get_matches();
  match matches.subcommand() {
    Some(("build", _)) => {
      build();
    }
    _ => {
      panic!("Unknown command {:?}", matches.subcommand().map(|c| c.0));
    }
  }
}
