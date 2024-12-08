use std::ops::{Add, AddAssign, Sub, SubAssign};

// TODO: Export general macros into tilc_macro_rules
// macro_rules! impl_into_for_definition {
//   ($ident:ident, $attr:tt; $($ty:ty),+) => {
//     $(
//       impl Into<$ty> for $ident {
//         fn into(self) -> $ty {
//           return self.$attr as $ty;
//         }
//       }
//     )+
//   };
// }
// impl_into_for_definition!(BytePos, 0; u8, u16, u32, u64, u128, usize);

pub trait Pos {
  fn from_u32(offset: u32) -> Self;
  fn to_u32(&self) -> u32;

  fn from_usize(offset: usize) -> Self;
  fn to_usize(&self) -> usize;
}

macro_rules! impl_pos_for_definition {
  ($(
      $(#[derive($($derives:ident),*)])*
      $vis:vis struct $name:ident($inner_vis:vis $inner_ty:ty);
    )*
  ) => {
    $(
      $(#[derive($($derives),*)])*
      $vis struct $name($inner_vis $inner_ty);
      impl Pos for $name {
        fn from_u32(offset: u32) -> Self {
          return Self(offset as $inner_ty);
        }
        fn to_u32(&self) -> u32 {
          return self.0 as u32;
        }

        fn from_usize(offset: usize) -> Self {
          return Self(offset as $inner_ty);
        }
        fn to_usize(&self) -> usize {
          return self.0 as usize;
        }
      }
      impl Into<$inner_ty> for $name {
        fn into(self) -> $inner_ty {
          return self.0 as $inner_ty;
        }
      }
      impl From<$inner_ty> for $name {
        fn from(offset: $inner_ty) -> Self {
          return Self(offset);
        }
      }

      impl Add for $name {
        type Output = Self;
        fn add(self, rhs: Self) -> Self {
          return Self(self.0 + rhs.0);
        }
      }
      impl AddAssign for $name {
        fn add_assign(&mut self, rhs: Self) {
          self.0 = self.0 + rhs.0;
        }
      }

      impl Sub for $name {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self {
          return Self(self.0 - rhs.0);
        }
      }
      impl SubAssign for $name {
        fn sub_assign(&mut self, rhs: Self) {
          self.0 = self.0 + rhs.0;
        }
      }
    )*
  };
}

impl_pos_for_definition! {
  #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
  pub struct BytePos(pub u32);


  // NOTE: For handling UTF-8 multi byte chars.
  //       BytePos(1) may not be equal to CharPos(1)
  // TODO: Implement this :)
  #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
  pub struct CharPos(pub u32);
}

// #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
// pub struct BytePos(pub u32);
// impl BytePos {
//   pub fn new(offset: u32) -> Self {
//     return Self(offset);
//   }
// }


// impl Add for BytePos {
//   type Output = Self;

//   fn add(self, rhs: Self) -> Self {
//     return Self(self.0 + rhs.0);
//   }
// }
// impl AddAssign for BytePos {
//   fn add_assign(&mut self, rhs: Self) {
//     self.0 = self.0 + rhs.0;
//   }
// }

// impl Sub for BytePos {
//   type Output = Self;

//   fn sub(self, rhs: Self) -> Self {
//     return Self(self.0 - rhs.0);
//   }
// }
// impl SubAssign for BytePos {
//   fn sub_assign(&mut self, rhs: Self) {
//     self.0 = self.0 - rhs.0;
//   }
// }
