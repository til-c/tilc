mod edition;
mod file_loader;
mod interner;
mod module;
mod pos;
mod session;
mod source_map;
mod span;
mod symbol;


pub use edition::*;
pub use file_loader::*;
pub use interner::*;
pub use module::*;
pub use pos::*;
pub use session::*;
pub use source_map::*;
pub use span::*;
pub use symbol::*;


// pub static SESSION_GLOBALS: SessionGlobals = SessionGlobals::new();
thread_local! {
  static SESSION_GLOBALS: SessionGlobals = SessionGlobals::new();
}
// scoped_tls::scoped_thread_local!(static SESSION_GLOBALS: SessionGlobals);


// pub fn init_session_globals<R, F>(t: &SessionGlobals, f: F) -> R
// where
//   F: FnOnce() -> R, {
//   debug_assert!(
//     !SESSION_GLOBALS.is_set(),
//     "SESSION_GLOBALS must not be overwriten"
//   );
//   return SESSION_GLOBALS.set(t, f);
// }
pub fn with_session_globals<R, F>(f: F) -> R
where
  F: FnOnce(&SessionGlobals) -> R, {
  return SESSION_GLOBALS.with(f);
}
