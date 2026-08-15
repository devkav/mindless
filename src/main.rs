mod util;
mod commands;
mod objects;

use clap::{CommandFactory, Parser, Subcommand};
use commands::save::save;
use commands::new::new;
use commands::history::history;
use commands::diff::diff;

use crate::util::files::decompress_file;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Debug tool to decompress files
    DbgDecompress {
        path: String,
    },
    /// Save the state of the current workspace
    Save {
        message: String
    },
    /// Create a new mindless project
    New {},
    /// View change history of the current workspace
    History {},
    /// View file changes
    Diff {},
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::DbgDecompress { path }) => {
            let decompressed = decompress_file(&path);
            println!("{}", decompressed)
        }
        Some(Commands::Save { message }) => save(message),
        Some(Commands::New {}) => new(),
        Some(Commands::History {}) => history(),
        Some(Commands::Diff {}) => diff(),
        None => {
            let _ = Cli::command().print_help();
        }
    }
}
