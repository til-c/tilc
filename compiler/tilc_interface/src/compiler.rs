use tilc_backend::Backend;
use tilc_session::Session;


pub struct Compiler {
  session: Session,
  backend: Box<dyn Backend>,
}
impl Compiler {
  pub fn new(session: Session, backend: Box<dyn Backend>) -> Self {
    return Self { session, backend };
  }
}
