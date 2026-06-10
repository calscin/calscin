use crate::commands::CLICommand;
use clap::Parser;

pub mod commands;

#[derive(Parser)]
pub struct CliParser {
    #[command(subcommand, name = "calscin")]
    pub command: CLICommand,
}

fn main() {
    let cli = CliParser::parse();
}
