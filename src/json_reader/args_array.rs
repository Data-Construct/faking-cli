use serde::{Serialize, Deserialize};
use super::args_null::NullArgs;

#[derive(Serialize, Deserialize, Debug)]
pub struct ArrayArgs {
  pub active: bool,
  pub min: u64,
  pub max: u64,
  pub null: NullArgs,
}