use std::{path::PathBuf, rc::Rc};

pub struct SourceFile {
  pub name: PathBuf,
  pub src: Rc<String>,

  /// For the package manager to check whether the external package is damaged or new/old
  pub _checksum_hash: bool, // TODO: Implement this feather
}
