use std::{panic, path::PathBuf};

use tilc_errors::{FatalError, FatalErrorMarker};


pub fn catch_if_error<F, R>(f: F) -> Result<R, FatalError>
where
  F: FnOnce() -> R, {
  panic::catch_unwind(panic::AssertUnwindSafe(f)).map_err(|value| {
    if value.is::<FatalErrorMarker>() {
      FatalError
    } else {
      panic::resume_unwind(value);
    }
  })
}

/// FIXME: HARDCODED funciton
///
/// I guess I can do it better
pub fn get_sys_root_path() -> PathBuf {
  #[cfg(not(any(target_os = "windows")))]
  fn sys_root() -> PathBuf {
    panic!()
  }
  #[cfg(target_os = "windows")]
  fn sys_root() -> PathBuf {
    return PathBuf::from("C:/WINDOWS");
  }


  return sys_root();
}


pub fn get_backend() {}
