extern crate lazy_static;

use clap::{error, Parser, Subcommand};
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
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => {} // println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Test { list }) => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        None => {}
    }

    if let Some(config_path) = cli.config.as_deref() {
        // println!("Value for config: {}", config_path.display());
        let vec_res = match file_reader::read(config_path) {
            Ok(res) => res,
            Err(error) => return println!("{}", error),
        };

        let struct_res = match deserialize(vec_res) {
            Ok(res) => res,
            Err(error) => return println!("{}", error),
        };

        // println!("{:#?}", struct_res);
        // println!("\n\n\n");

        generate_json_output(&struct_res, 3);
    }
}
