use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NullArgs {
  pub str: String,
  pub percentage: u32,
}