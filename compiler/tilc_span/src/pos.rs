use std::ops::{Add, Sub};

pub struct Pos(pub u32);
impl Pos {
  pub fn new(offset: u32) -> Self {
    return Self(offset);
  }
}
impl Add for Pos {
  type Output = Self;

  fn add(self, rhs: Self) -> Self {
    return Self(self.0 + rhs.0);
  }
}
impl Sub for Pos {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self {
    return Self(self.0 - rhs.0);
  }
}
