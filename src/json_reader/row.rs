use serde::{Serialize, Deserialize};
use super::{args_null::NullArgs, args_array::ArrayArgs};

#[derive(Serialize, Deserialize, Debug)]
pub struct GeneratorArgs {
  // TBD
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Row {
  pub field_name: String,
  pub generator: String,
  pub generator_arguments: Option<GeneratorArgs>,
  pub null: NullArgs,
  pub array: ArrayArgs
}