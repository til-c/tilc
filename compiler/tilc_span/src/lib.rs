mod def_id;
mod edition;
mod interner;
mod pos;
mod session;
mod source_map;
mod symbol;

pub use def_id::*;
pub use edition::*;
pub use interner::*;
pub use pos::*;
pub use session::*;
pub use source_map::*;
pub use symbol::*;

use std::path::PathBuf;


thread_local! {
  pub static SESSION_GLOBALS: SessionGlobals = SessionGlobals::new();
}
pub fn with_session_globals<R, F>(f: F) -> R
where
  F: FnOnce(&SessionGlobals) -> R, {
  return SESSION_GLOBALS.with(f);
}

#[derive(Debug, Clone, Copy)]
pub struct ErrorGuaranteed(());
impl ErrorGuaranteed {
  pub fn new_unchecked() -> Self {
    return Self(());
  }
}


#[derive(Debug, Hash)]
pub enum RealFileName {
  Local(PathBuf),
  Remapped {
    local_path: Option<PathBuf>,
    virtual_path: PathBuf,
  },
}
#[derive(Debug, Hash)]
pub enum FileName {
  Real(RealFileName),
  // TODO: make Hash struct for simple u64 hashing
  Anon(),
}
