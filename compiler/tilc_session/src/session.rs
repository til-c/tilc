use std::path::PathBuf;

use crate::ParseSession;

#[derive(Debug)]
pub enum Input {
  File(PathBuf),
}
#[derive(Debug)]
pub struct Session {
  pub input_file: Input,
  pub psess: ParseSession,
}
