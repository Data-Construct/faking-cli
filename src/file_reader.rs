use std::{fs::File, io::{self, Read}, path::Path};
use io::{Error, ErrorKind};

fn is_json(path: &Path) -> bool {
  let ext_option = path.extension();
  if ext_option.is_none() {
    return false;
  }

  let ext = ext_option.unwrap();
  ext.to_str() == Some("json")
}

pub fn read(path: &Path) -> Result<Vec<u8>, Error> {
  if !path.exists() {
    return Err(Error::new(ErrorKind::NotFound, "Provided path does not exist."));
  }

  if path.is_dir() {
    return Err(Error::new(ErrorKind::InvalidInput, "Path provided is directory, not file."));
  }

  if !is_json(path) {
    return Err(Error::new(ErrorKind::InvalidInput, "Provided path points to non-JSON file."));
  }

  let mut file = match File::open(path) {
      Ok(file) => file,
      Err(error) => return Err(error)
  };

  let mut buf = Vec::new();
  file.read_to_end(&mut buf);

  Ok(buf)
}