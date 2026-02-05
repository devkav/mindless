use std::fs;
use clap::{Parser, Subcommand};
use zlib_rs::{InflateConfig, compress_bound, decompress_slice};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    name: Option<String>,

    #[arg(short, long, value_name = "FILE")]
    config: Option<std::path::PathBuf>,

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
    }
}

fn main() {
    /*
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
        None => {}
    }
    */

    let bytes: Vec<u8> = fs::read("./src/example/blob").expect("Something went wrong");
    let input: &[u8] = &bytes;
    let mut decompressed_buf = vec![0u8; 1247];

    let (decompressed, rc) = decompress_slice(&mut decompressed_buf, input, InflateConfig::default());
    let output_result = str::from_utf8(decompressed);

    if let Ok(output) = output_result {
        println!("{}", output);
    }


    //let byte_string = 
    //let mut decompressed_buf = vec![0u8, compress_bound(bytes.len())]
}
