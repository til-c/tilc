use std::path::PathBuf;

use crate::{Input, Options, ParseSession};

pub struct Target {
  triplet: String,
  architecture: String,
}
pub struct IO {
  input: Input,
  output_dir: PathBuf,
  output_file: PathBuf,
}


pub struct Session {
  target: Target,
  opts: Options,
  parse_session: ParseSession,
  root: PathBuf,
  io: IO,
}
impl Session {}
