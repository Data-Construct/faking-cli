extern crate lazy_static;

use clap::{Parser, Subcommand};
use data_faking_bridge::assoc::get_func_from_string;
use output_formats::json::generate_json_output;
use solo_generator::produce_generator_value;
use std::path::PathBuf;

use crate::json_reader::deserialize::deserialize;

mod data_faking_bridge;
mod file_reader;
mod json_reader;
mod list_subcommand;
mod output_formats;
mod solo_generator;

// use output_formats::json::GenerateJSONOutput;

const DEFAULT_GEN_AMOUNT: i64 = 3;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Generator string
    #[arg(index = 1)]
    generator: Option<String>,

    /// Number of generations to output - 3 by default
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
        category: Option<String>,

        #[arg(short, long)]
        show_categories: bool,
    },
}

fn main() -> Result<(), std::io::Error> {
    let cli = Cli::parse();

    // if cli.debug {
    //     println!("Debugging turned on");
    // }

    match &cli.generator {
        Some(gstr) => {
            // println!("{}", gstr);
            let x = get_func_from_string(gstr);

            let mut gen_amount = DEFAULT_GEN_AMOUNT;
            match cli.n {
                Some(val) => {
                    gen_amount = val;
                    if gen_amount < 1 {
                        gen_amount = 1
                    }
                }

                None => {}
            }

            let mut oc = String::from("");
            for i in 0..gen_amount {
                produce_generator_value(&mut oc, &x);

                if i != gen_amount - 1 {
                    oc.push('\n');
                }
            }
            println!("{}", oc);

            return Ok(());
        }

        None => {}
    }

    match &cli.command {
        Some(Commands::List {
            category,
            show_categories,
        }) => {
            if *show_categories {
                list_subcommand::print_categories();
                return Ok(());
            }

            match category {
                Some(cat) => {
                    list_subcommand::print_category(cat);
                    return Ok(());
                }

                None => {
                    list_subcommand::print_all();
                    return Ok(());
                }
            }
        }

        None => {}
    }

    if let Some(config_path) = cli.schema.as_deref() {
        // println!("Value for config: {}", config_path.display());

        let vec_res = match file_reader::read(config_path) {
            Ok(res) => res,
            Err(error) => return Err(error),
        };

        let struct_res = match deserialize(vec_res) {
            Ok(res) => res,
            Err(error) => return Err(error),
        };

        let mut n_rows = DEFAULT_GEN_AMOUNT;

        match struct_res.meta.as_ref() {
            Some(meta) => match meta.number {
                Some(number) => {
                    n_rows = number;

                    if n_rows < 1 {
                        n_rows = 1
                    }
                }
                None => {}
            },

            None => {}
        }

        match cli.n {
            Some(val) => {
                n_rows = val;
                if n_rows < 1 {
                    n_rows = 1
                }
            }

            None => {}
        }

        generate_json_output(&struct_res, n_rows);
    }

    return Ok(());
}
