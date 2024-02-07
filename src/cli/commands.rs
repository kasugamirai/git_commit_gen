use clap::{Arg, Command};

pub fn build_cli_app() -> Command {
    Command::new("git-gen")
        .version("0.1.0")
        .author("XY01 xyzmhx@gmail.com")
        .about("Generates Git commit messages using GPT")
        .arg(
            Arg::new("generate")
                .short('g')
                .long("generate")
                .action(clap::ArgAction::SetTrue)
                .help("Generates a Git commit message"),
        )
}
