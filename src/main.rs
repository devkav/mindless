mod util;
mod commands;

use clap::{Parser, Subcommand};
use util::blob;
use commands::save;


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
    HelloWorld {
        #[arg(short, long)]
        number: Option<u8>,
    },
    DbgDecompress {
        path: String,
    },
    Save {
        message: String
    }
}

fn main() {
    let cli = Cli::parse();

    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        _ => println!("Debug mode is fully on"),
    }

    match cli.command {
        Some(Commands::HelloWorld { number }) => {
            let mut print_count = 1;

            if let Some(selected_number) = &number {
                print_count = *selected_number;
            }

            // Remove mutability
            let print_count = print_count;

            for _ in 0..print_count {
                println!("Hello world!");
            }
        }
        Some(Commands::DbgDecompress { path }) => {
            println!("{}", path);
            let decompressed = blob::decompress_blob(path);
            println!("{}", decompressed)
        }
        Some(Commands::Save { message }) => save(message),
        None => {}
    }
}
