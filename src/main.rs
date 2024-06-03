extern crate lazy_static;

use clap::{Parser, Subcommand};
use output_formats::json::generate_json_output;
use std::path::PathBuf;

use crate::json_reader::deserialize::deserialize;

mod data_faking_bridge;
mod file_reader;
mod json_reader;
mod output_formats;

// use output_formats::json::GenerateJSONOutput;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional number of rows - 3 by default
    #[arg(short)]
    n: Option<i64>,

    /// Use schema file for generation
    #[arg(short, long, value_name = "FILE")]
    schema: Option<PathBuf>,

    // /// Turn debugging information on
    // #[arg(short, long, action)]
    // debug: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Prints all generator strings that can be used.
    List {
        #[arg(short, long)]
        list: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    // if cli.debug {
    //     println!("Debugging turned on");
    // }

    match &cli.command {
        Some(Commands::List { list }) => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        None => {}
    }

    if let Some(config_path) = cli.schema.as_deref() {
        // println!("Value for config: {}", config_path.display());

        let vec_res = match file_reader::read(config_path) {
            Ok(res) => res,
            Err(error) => return println!("{}", error),
        };

        let struct_res = match deserialize(vec_res) {
            Ok(res) => res,
            Err(error) => return println!("{}", error),
        };

        match cli.n {
            Some(val) => {
                let mut n = val;
                if n < 1 {
                    n = 1
                }

                generate_json_output(&struct_res, n);
            }
            None => {
                generate_json_output(&struct_res, 3);
            }
        }
    }
}
