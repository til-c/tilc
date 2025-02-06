use tilc_backend::Backend;
use tilc_session::Session;

use crate::Computation;


pub struct Compiler {
  pub session: Session,
  pub backend: Box<dyn Backend>,
}
impl Compiler {
  pub fn new(session: Session, backend: Box<dyn Backend>) -> Self {
    return Self { session, backend };
  }


  pub fn enter<T, F: for<'cpl> FnOnce(&'cpl Computation<'cpl>) -> T>(
    &self,
    f: F,
  ) -> T {
    let computation: Computation<'_> = Computation::new(self);
    return f(&computation);
  }
}
