use std::io;
use super::obj::Obj;

pub fn deserialize(json: Vec<u8>) -> Result<Obj, io::Error> {
  let res = String::from_utf8(json);
  if res.is_err() {
    let msg = format!("Unable to convert JSON to UTF-8: {}", res.err().unwrap().to_string());
    return Err(io::Error::new(io::ErrorKind::InvalidData, msg));
  }

  let s = res.ok().unwrap();
  let m: Obj = {
    serde_json::from_str(&s).unwrap()
  };
  Ok(m)
}