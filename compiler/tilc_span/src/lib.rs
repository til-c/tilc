mod edition;
mod file_loader;
mod interner;
pub mod keyword;
mod module;
mod pos;
mod session;
mod source_map;
mod span;
mod symbol;


pub use edition::*;
pub use file_loader::*;
pub use interner::*;
pub use keyword::*;
pub use module::*;
pub use pos::*;
pub use session::*;
pub use source_map::*;
pub use span::*;
pub use symbol::*;


use tilc_data_structures::Hash64;


thread_local! {
  static SESSION_GLOBALS: SessionGlobals = SessionGlobals::new();
}
pub fn with_session_globals<R, F>(f: F) -> R
where
  F: FnOnce(&SessionGlobals) -> R, {
  return SESSION_GLOBALS.with(f);
}


/// Struct for handling errors and representing the proof that error has been consumed
#[derive(Clone, Copy, Debug)]
pub struct ErrorGuaranteed(());
impl ErrorGuaranteed {
  pub fn new_unchecked() -> Self {
    return Self(());
  }
}


#[derive(Hash)]
pub struct SandyqId(Hash64);
