mod diagnostic;


pub use diagnostic::*;


pub struct FatalErrorMarker;
pub struct FatalError;
impl FatalError {
  pub fn raise(self) -> ! {
    std::panic::resume_unwind(Box::new(FatalErrorMarker))
  }
}
