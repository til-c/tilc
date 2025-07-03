mod pos;

pub use pos::*;

use std::path::PathBuf;

#[derive(Debug, Clone, Copy)]
pub struct ErrorGuaranteed(());
impl ErrorGuaranteed {
  pub fn new_unchecked() -> Self {
    return Self(());
  }
}


#[derive(Debug)]
pub enum RealFileName {
  Local(PathBuf),
  Remapped {
    local_path: Option<PathBuf>,
    virtual_path: PathBuf,
  },
}
#[derive(Debug)]
pub enum FileName {
  Real(RealFileName),
  // TODO: make Hash struct for simple u64 hashing
  Anon(),
}
