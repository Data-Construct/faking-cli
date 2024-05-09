use serde::{Serialize, Deserialize};
use super::{args_null::NullArgs, args_array::ArrayArgs, row::Row};

#[derive(Serialize, Deserialize, Debug)]
enum FieldsEnum {
  Obj(Obj),
  Row(Row)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Obj {
  pub id: String,
  pub depth: u32,
  pub expanded: bool,
  pub field_name: String,
  pub null: NullArgs,
  pub fields: Vec<FieldsEnum>
}