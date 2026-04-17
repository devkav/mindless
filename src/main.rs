mod util;
mod commands;

use clap::{CommandFactory, Parser, Subcommand};
use util::blob;
use commands::save::save;
use commands::init::init;


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
            println!("{}", path);
            let decompressed = blob::decompress_blob(path);
            println!("{}", decompressed)
        }
        Some(Commands::Save { message }) => save(message),
        Some(Commands::Init {}) => init(),
        None => {
            let _ = Cli::command().print_help();
        }
    }
}
