mod util;
mod commands;
mod objects;

use clap::{CommandFactory, Parser, Subcommand};
use commands::save::save;
use commands::init::init;

use crate::util::files::decompress_file;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    DbgDecompress {
        path: String,
    },
    Save {
        message: String
    },
    Init {}
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::DbgDecompress { path }) => {
            let decompressed = decompress_file(path);
            println!("{}", decompressed)
        }
        Some(Commands::Save { message }) => save(message),
        Some(Commands::Init {}) => init(),
        None => {
            let _ = Cli::command().print_help();
        }
    }
}
