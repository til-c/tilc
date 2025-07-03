pub struct FatalErrorMarker;
pub struct FatalError;
impl FatalError {
  pub fn raise() -> ! {
    std::panic::resume_unwind(Box::new(FatalErrorMarker));
  }
}
