#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Hash64(u64);
impl Hash64 {
  pub const DUMMY: Self = Self(0);

  pub fn new(n: u64) -> Self {
    return Self(n);
  }
  pub fn to_u64(&self) -> u64 {
    return self.0;
  }
}
