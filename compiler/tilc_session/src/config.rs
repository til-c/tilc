use std::path::PathBuf;

pub enum Input {
  File(PathBuf),
  Str(String),
}


pub struct Options {}
