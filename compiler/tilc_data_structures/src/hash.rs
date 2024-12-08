use std::hash;


#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Hash64(u64);
impl Hash64 {
  pub const EMPTY: Hash64 = Hash64(0);


  pub fn new(n: u64) -> Self {
    return Self(n);
  }
  pub fn as_u64(&self) -> u64 {
    return self.0;
  }
}

#[derive(Clone, Hash)]
pub struct Hash128(u128);
impl Hash128 {
  fn truncate(&self) -> Hash64 {
    return Hash64(self.0 as u64);
  }
  fn as_u128(&self) -> u128 {
    return self.0;
  }
}
// impl hash::Hash for Hash128 {
//   fn hash<H: hash::Hasher>(&self, h: &mut H) {
//     h.write_u64(self.truncate().as_u64());
//   }
// }
