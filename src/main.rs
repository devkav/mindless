mod util;
mod commands;

use clap::{CommandFactory, Parser, Subcommand};
use util::blob;
use commands::save;

use crate::commands::init;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    name: Option<String>,

    // #[arg(short, long, value_name = "FILE")]
    // config: Option<std::path::PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

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

    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    /*
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        _ => println!("Debug mode is fully on"),
    }
    */

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
