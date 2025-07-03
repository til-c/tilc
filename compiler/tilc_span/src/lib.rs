#[derive(Debug, Clone, Copy)]
pub struct ErrorGuaranteed(());
impl ErrorGuaranteed {
  pub fn new_unchecked() -> Self {
    return Self(());
  }
}
