#[macro_use]
extern crate lazy_static;

use clap::{error, Parser, Subcommand};
use json_reader::{
    obj::{DTObj, FieldsEnum},
    row::Row,
};
use rand::Rng;
use std::path::PathBuf;

use crate::{
    data_faking_bridge::assoc::{get_func_from_string, FNVARI},
    json_reader::deserialize::deserialize,
};

mod data_faking_bridge;
mod file_reader;
mod json_reader;

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

        let _x = GenerateJSONOutput(&struct_res, 3);
    }
}

fn GenerateJSONOutput(grs: &DTObj, n_rows: i64) -> i32 {
    let mut content = "[\n".to_owned();

    for i in 0..n_rows {
        content.push_str(&LoopThroughNestedObj(&grs.schema, 0));

        if i != n_rows - 1 {
            content.push_str(",\n");
        }
    }

    content.push_str("\n]");

    println!("{}", content);
    // println!("{:#?}", content);

    // return content;
    0
}

fn LoopThroughNestedObj(rows: &Vec<FieldsEnum>, depth: i16) -> String {
    let mut oc = "".to_owned();
    if depth >= 1 {
        oc.push_str("{\n");
    } else {
        oc.push_str("\t{\n");
    }

    for i in 0..rows.len() {
        oc.push_str("\t\t");
        for a in 0..depth {
            oc.push_str("\t");
        }

        match &rows[i] {
            FieldsEnum::Obj(obj) => {
                oc.push_str("\"");
                oc.push_str(&obj.field_name);
                oc.push_str("\": ");

                if obj.array.active {
                    if obj.null.percentage == 0 {
                        let arr_range = rand::thread_rng().gen_range(obj.array.min..obj.array.max);

                        oc.push_str("[\n");
                        oc.push_str("\t\t\t");
                        for a in 0..depth {
                            oc.push_str("\t");
                        }

                        for a in 0..arr_range {
                            if obj.array.null.percentage == 0 {
                                oc.push_str(LoopThroughNestedObj(&obj.fields, depth + 2).as_str());
                            } else {
                                let rand_roll = rand::thread_rng().gen_range(0..100);
                                if obj.array.null.percentage < rand_roll {
                                    oc.push_str(
                                        LoopThroughNestedObj(&obj.fields, depth + 2).as_str(),
                                    );
                                } else {
                                    insert_null_or_custom_string(&mut oc, &obj.array.null.str);
                                }
                            }

                            if a != arr_range - 1 {
                                oc.push_str(",\n");

                                oc.push_str("\t\t\t");
                                for a in 0..depth {
                                    oc.push_str("\t");
                                }
                            }
                        }

                        oc.push_str("\n\t\t");
                        for a in 0..depth {
                            oc.push_str("\t");
                        }
                        oc.push_str("]");
                    } else {
                        let rand_roll = rand::thread_rng().gen_range(0..100);
                        if obj.null.percentage < rand_roll {
                            let arr_range =
                                rand::thread_rng().gen_range(obj.array.min..obj.array.max);

                            oc.push_str("[\n");
                            oc.push_str("\t\t\t");
                            for a in 0..depth {
                                oc.push_str("\t");
                            }

                            for a in 0..arr_range {
                                if obj.array.null.percentage == 0 {
                                    oc.push_str(
                                        LoopThroughNestedObj(&obj.fields, depth + 2).as_str(),
                                    );
                                } else {
                                    let rand_roll = rand::thread_rng().gen_range(0..100);
                                    if obj.array.null.percentage < rand_roll {
                                        oc.push_str(
                                            LoopThroughNestedObj(&obj.fields, depth + 2).as_str(),
                                        );
                                    } else {
                                        insert_null_or_custom_string(&mut oc, &obj.array.null.str);
                                    }
                                }

                                if a != arr_range - 1 {
                                    oc.push_str(",\n");

                                    oc.push_str("\t\t\t");
                                    for a in 0..depth {
                                        oc.push_str("\t");
                                    }
                                }
                            }

                            oc.push_str("\n\t\t");
                            for a in 0..depth {
                                oc.push_str("\t");
                            }
                            oc.push_str("]");
                        } else {
                            insert_null_or_custom_string(&mut oc, &obj.null.str);
                        }
                    }
                } else {
                    if obj.null.percentage == 0 {
                        oc.push_str(LoopThroughNestedObj(&obj.fields, depth + 1).as_str());
                    } else {
                        let rand_roll = rand::thread_rng().gen_range(0..100);
                        if obj.null.percentage < rand_roll {
                            oc.push_str(LoopThroughNestedObj(&obj.fields, depth + 1).as_str());
                        } else {
                            insert_null_or_custom_string(&mut oc, &obj.null.str);
                        }
                    }
                }

                if i != rows.len() - 1 {
                    oc.push_str(",\n");
                }
            }

            FieldsEnum::Row(row) => {
                oc.push_str("\"");
                oc.push_str(&row.field_name);
                oc.push_str("\": ");

                if row.array.active {
                    if row.null.percentage == 0 {
                        let arr_range = rand::thread_rng().gen_range(row.array.min..row.array.max);

                        oc.push_str("[");

                        for a in 0..arr_range {
                            if row.array.null.percentage == 0 {
                                oc.push_str(CreateFieldString(&row.generator).as_str());
                            } else {
                                let rand_roll = rand::thread_rng().gen_range(0..100);
                                if row.array.null.percentage < rand_roll {
                                    oc.push_str(CreateFieldString(&row.generator).as_str());
                                } else {
                                    insert_null_or_custom_string(&mut oc, &row.array.null.str);
                                }
                            }

                            if a != arr_range - 1 {
                                oc.push_str(", ");
                            }
                        }

                        oc.push_str("]");
                    } else {
                        let rand_roll = rand::thread_rng().gen_range(0..100);
                        if row.null.percentage < rand_roll {
                            let arr_range =
                                rand::thread_rng().gen_range(row.array.min..row.array.max);

                            oc.push_str("[");

                            for a in 0..arr_range {
                                if row.array.null.percentage == 0 {
                                    oc.push_str(CreateFieldString(&row.generator).as_str());
                                } else {
                                    let rand_roll = rand::thread_rng().gen_range(0..100);
                                    if row.array.null.percentage < rand_roll {
                                        oc.push_str(CreateFieldString(&row.generator).as_str());
                                    } else {
                                        insert_null_or_custom_string(&mut oc, &row.array.null.str);
                                    }
                                }

                                if a != arr_range - 1 {
                                    oc.push_str(", ");
                                }
                            }

                            oc.push_str("]");
                        } else {
                            insert_null_or_custom_string(&mut oc, &row.null.str);
                        }
                    }
                } else {
                    if row.null.percentage == 0 {
                        oc.push_str(CreateFieldString(&row.generator).as_str());
                    } else {
                        let rand_roll = rand::thread_rng().gen_range(0..100);
                        if row.null.percentage < rand_roll {
                            oc.push_str(CreateFieldString(&row.generator).as_str());
                        } else {
                            insert_null_or_custom_string(&mut oc, &row.null.str);
                        }
                    }
                }

                if i != rows.len() - 1 {
                    oc.push_str(",\n");
                }
            }
        }
    }

    oc.push_str("\n");
    oc.push_str("\t");
    for a in 0..depth {
        oc.push_str("\t");
    }
    oc.push_str("}");

    oc
}

#[inline(always)]
fn insert_null_or_custom_string(oc: &mut String, null_string: &Option<String>) {
    match null_string {
        Some(x) => {
            oc.push_str("\"");
            oc.push_str(x.as_str());
            oc.push_str("\"");
        }
        None => oc.push_str("null"),
    }
}

fn CreateFieldString(rs: &String) -> String {
    let mut oc = "".to_owned();

    // TODO(clearfeld): do this once and save the FN_E pointer instead of always doing the lookup
    // this function shouldn't need to take in a &String at all
    let x = get_func_from_string(rs);

    match x {
        FNVARI::I8(f) => {
            oc.push_str(f().to_string().as_str());
        }

        FNVARI::Bool(f) => {
            oc.push_str(f().to_string().as_str());
        }

        FNVARI::String(f) => {
            oc.push_str("\"");
            oc.push_str(f().as_str());
            oc.push_str("\"");
        }
    }

    oc
}
