use std::{thread, time::Duration};

use crate::commands::CLICommand;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};

pub mod commands;

#[derive(Parser)]
pub struct CliParser {
    #[command(subcommand, name = "calscin")]
    pub command: CLICommand,
}

fn main() {
    let cli = CliParser::parse();
}
