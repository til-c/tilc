use std::path::PathBuf;

use crate::Options;

pub enum Input {
  File(PathBuf),
  Str(String),
}


pub struct Config {
  pub options: Options,
  pub input: Input,

  pub raw_args: Vec<String>,
}
