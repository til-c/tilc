use std::panic;

use tilc_error::FatalError;

pub fn catch_if_error<F, R>(f: F) -> Result<R, FatalError>
where
  F: FnOnce() -> R, {
  return panic::catch_unwind(panic::AssertUnwindSafe(f)).map_err(|value| {
    if value.is::<FatalError>() {
      return FatalError;
    } else {
      panic::resume_unwind(value);
    };
  });
}
