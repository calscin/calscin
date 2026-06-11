use crate::commands::{
    CLICommand, build::build_command, check::check_command, remir::remir_command,
};
use clap::Parser;

pub mod commands;

#[derive(Parser)]
pub struct CliParser {
    #[command(subcommand, name = "calscin")]
    pub command: CLICommand,
}

fn main() {
    let cli = CliParser::parse();

    match cli.command {
        CLICommand::Build {
            input,
            out,
            linker,
            use_pie,
        } => build_command(input, out, linker, use_pie),

        CLICommand::Check { input, simple } => check_command(input, simple),

        CLICommand::Remir { input } => remir_command(input),
    }
}
