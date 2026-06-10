use std::path::PathBuf;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum CLICommand {
    #[command(
        visible_alias = "b",
        about = "Builds the given Calscin files into an executable"
    )]
    Build {
        #[arg(required = true)]
        input: Vec<PathBuf>,

        #[arg(short = 'o')]
        out: PathBuf,

        #[arg(short = 'l', default_value = "ld")]
        linker: String,
    },
}
