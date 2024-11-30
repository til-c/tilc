use std::path::PathBuf;

use crate::{Input, Options, ParseSession};

pub struct Target {
  pub triplet: String,
  pub architecture: String,
}
pub struct IO {
  pub input: Input,
  pub output_dir: PathBuf,
  pub output_file: PathBuf,
}


pub struct Session {
  pub target: Target,
  pub opts: Options,
  pub parse_session: ParseSession,
  pub sys_root: PathBuf,
  pub io: IO,
}
impl Session {}
