pub mod interface;


use std::panic;

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

pub struct Compiler<'a> {
  args: &'a [String],
}
impl<'a> Compiler<'a> {
  pub fn new(args: &'a [String]) -> Self {
    return Self { args };
  }

  pub fn run(&self) -> interface::Result<()> {
    todo!()
  }
}
