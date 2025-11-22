mod interner;
mod keywords;
mod session;
mod symbol;

pub use interner::*;
pub use keywords::*;
pub use session::*;
pub use symbol::*;

thread_local! {
  static SESSION_GLOBALS: SessionGlobals = SessionGlobals::new();
}
pub fn with_session_globals<F, R>(f: F) -> R
where
  F: FnOnce(&SessionGlobals) -> R, {
  return SESSION_GLOBALS.with(f);
}
