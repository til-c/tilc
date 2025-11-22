mod def_id;
mod edition;
mod filename;
mod interner;
mod keywords;
mod pos;
mod session;
mod source_map;
mod span;
mod symbol;

pub use def_id::*;
pub use edition::*;
pub use filename::*;
pub use interner::*;
pub use keywords::*;
pub use pos::*;
pub use session::*;
pub use source_map::*;
pub use span::*;
pub use symbol::*;

thread_local! {
  static SESSION_GLOBALS: SessionGlobals = SessionGlobals::new();
}
pub fn with_session_globals<F, R>(f: F) -> R
where
  F: FnOnce(&SessionGlobals) -> R, {
  SESSION_GLOBALS.with(f)
}
