#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, Eq)]
#[derive(Hash)]
pub struct Hash64(u64);
impl Hash64 {
  pub const EMPTY: Self = Self(0);

  pub const fn new(n: u64) -> Self {
    Self(n)
  }

  pub const fn to_u64(&self) -> u64 {
    self.0
  }
}
