macro_rules! impl_pos {
  ($(
    $(#[$attrs:meta])?
    $vis:vis struct $name:ident($inner_vis:vis $inner_ty:ty);
  )*) => {$(
    $(#[$attrs])?
    $vis struct $name($inner_vis $inner_ty);
    impl Pos for BytePos {
      #[inline(always)]
      fn from_u32(value: u32) -> Self {
        return Self(value as $inner_ty);
      }
      #[inline(always)]
      fn to_u32(&self) -> u32 {
        return self.0;
      }

      #[inline(always)]
      fn from_usize(value: usize) -> Self {
        return Self(value as $inner_ty);
      }
      #[inline(always)]
      fn to_usize(&self) -> usize {
        return self.0 as usize;
      }
    }

    impl std::ops::Add for $name {
      type Output = Self;

      #[inline(always)]
      fn add(self, rhs: Self) -> Self::Output {
        return Self(self.0 + rhs.0);
      }
    }
    impl std::ops::Sub for $name {
      type Output = Self;

      #[inline(always)]
      fn sub(self, rhs: Self) -> Self::Output {
        return Self(self.0 - rhs.0);
      }
    }
  )*};
}

pub trait Pos {
  fn from_u32(value: u32) -> Self;
  fn to_u32(&self) -> u32;

  fn from_usize(value: usize) -> Self;
  fn to_usize(&self) -> usize;
}

impl_pos! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
  pub struct BytePos(u32);
}
