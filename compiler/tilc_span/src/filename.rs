use std::path::PathBuf;

#[derive(Debug, Hash)]
pub enum Filename {
  Real(RealFileName),
  Anon,
}
#[derive(Debug, Hash)]
pub enum RealFileName {
  Local(PathBuf),
  Remapped {
    local_path: Option<PathBuf>,
    virtual_path: PathBuf,
  },
}
