macro_rules! impl_pos {
  ($(
    $(#[$attr:meta])*
    $vis:vis struct $name:ident($inner_vis:vis $inner_ty:ty);
  )*) => {$(
    $(#[$attr])*
    $vis struct $name($inner_vis $inner_ty);
    impl Pos for $name {
      #[inline(always)]
      fn from_u32(v: u32) -> Self {
        Self(v as $inner_ty)
      }
      #[inline(always)]
      fn to_u32(&self) -> u32 {
        self.0 as u32
      }

      #[inline(always)]
      fn from_usize(v: usize) -> Self {
        Self(v as $inner_ty)
      }
      #[inline(always)]
      fn to_usize(&self) -> usize {
        self.0 as usize
      }
    }

    impl ::std::ops::Add for $name {
      type Output = Self;

      #[inline(always)]
      fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
      }
    }
    impl ::std::ops::Sub for $name {
      type Output = Self;

      #[inline(always)]
      fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
      }
    }

    impl ::std::convert::From<$inner_ty> for $name {
      fn from(value: $inner_ty) -> Self {
        Self(value)
      }
    }
  )*};
}

pub trait Pos {
  fn from_u32(v: u32) -> Self;
  fn to_u32(&self) -> u32;

  fn from_usize(v: usize) -> Self;
  fn to_usize(&self) -> usize;
}

impl_pos! {
  #[derive(Debug)]
  #[derive(Clone, Copy)]
  #[derive(PartialEq, Eq)]
  #[derive(PartialOrd, Ord)]
  pub struct BytePos(pub u32);
}
