mod convert;
mod copy;

use clap::{Parser, Subcommand};
use convert::convert;

use crate::copy::copy;

const VERSION: &str = "0.1.0";
const ABOUT: &str =
    "Tools that I needed standardized because I hate jumping between linux and windows.";
const AUTHOR: &str = "reeeeedmil";
#[derive(Debug, clap::Args, Clone)]
#[group(required = true, multiple = false)]
pub struct ConvertGroup {
    /// Convert number to binary
    #[clap(short = 'b', long, value_name = "NUMBER")]
    binary: Option<u32>,
    /// Convert number to hexadecimal
    #[clap(short = 'x', long, value_name = "NUMBER")]
    hexadecimal: Option<u32>,
    /// Convert number to octal
    #[clap(short = 'o', long, value_name = "NUMBER")]
    octal: Option<u32>,
}

#[derive(Parser)]
#[command(author=AUTHOR, version=VERSION, about=ABOUT, long_about=None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}
#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Copies directory and everything inside it to a selected destination
    Copy {
        #[arg(short, long)]
        source_path: std::path::PathBuf,
        #[arg(short, long)]
        destination_path: std::path::PathBuf,
    },
    /// Converts decimal number to binary/hexadecimal/octal
    Convert {
        #[clap(flatten)]
        input: ConvertGroup,
    },
}

fn main() {
    let args = Args::parse();
    match args.cmd {
        Commands::Copy {
            source_path,
            destination_path,
        } => copy(source_path, destination_path),
        Commands::Convert { input } => convert(input),
    }
}
