use serde::{Serialize, Deserialize};
use super::{args_null::NullArgs, args_array::ArrayArgs, row::Row};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum FieldsEnum {
  Obj(Obj),
  Row(Row)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Obj {
  pub field_name: String,
  pub null: NullArgs,
  pub array: ArrayArgs,
  pub fields: Vec<FieldsEnum>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DTObj {
  // meta: Option<MetaObj>,
  pub schema: Vec<FieldsEnum>
}
