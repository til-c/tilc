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
  input: Input,

  output_dir: PathBuf,
  output_file: PathBuf,
}

#[derive(Debug)]
pub struct Session {
  io: CompilerIO,
  parse_session: ParseSession,
}
