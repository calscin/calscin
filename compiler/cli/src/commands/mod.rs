use std::path::PathBuf;

use clap::Subcommand;

pub mod build;
pub mod check;

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

        #[arg(
            short = 'l',
            default_value = "clang",
            help = "the linker used to assemble the object files"
        )]
        linker: String,

        #[arg(
            long,
            default_value_t = true,
            help = "should the compiler use PIE or not"
        )]
        use_pie: bool,
    },

    #[command(about = "Checks for errors without building the code")]
    Check {
        #[arg(required = true)]
        input: Vec<PathBuf>,

        #[arg(
            short = 's',
            help = "should the compiler only check for structure errors (simple errors)"
        )]
        simple: bool,
    },
}
