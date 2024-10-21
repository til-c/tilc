use std::ops::{Add, AddAssign, Sub, SubAssign};

// TODO: Export general macros into tilc_macro_rules
macro_rules! impl_into_for_definition {
  ($ident:ident, $attr:tt; $($ty:ty),+) => {
    $(
      impl Into<$ty> for $ident {
        fn into(self) -> $ty {
          return self.$attr as $ty;
        }
      }
    )+
  };
}
impl_into_for_definition!(Pos, 0; u8, u16, u32, u64, u128, usize);

#[derive(Clone, Copy)]
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
impl AddAssign for Pos {
  fn add_assign(&mut self, rhs: Self) {
    self.0 = self.0 + rhs.0;
  }
}

impl Sub for Pos {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self {
    return Self(self.0 - rhs.0);
  }
}
impl SubAssign for Pos {
  fn sub_assign(&mut self, rhs: Self) {
    self.0 = self.0 - rhs.0;
  }
}
