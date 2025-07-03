use std::path::PathBuf;

use tilc_span::FileName;

use crate::ParseSession;


#[derive(Debug)]
pub enum Input {
  File(PathBuf),
  String { name: FileName, input: String },
}
#[derive(Debug)]
pub struct CompilerIO {
  pub input: Input,

  pub output_dir: PathBuf,
  pub output_file: PathBuf,
}

#[derive(Debug)]
pub struct Session {
  pub io: CompilerIO,
  pub parse_session: ParseSession,
}
