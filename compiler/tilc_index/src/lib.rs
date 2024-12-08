#[macro_export]
macro_rules! uidx {
  ($($(#[derive($($derives:ident),*)])* $vis:vis struct $name:ident;)*) => {
    $(
      $(#[derive($($derives,)*)])*
      $vis struct $name(u32);
      impl $name {
        pub const EMPTY: Self = Self::new(u32::MAX);

        pub const fn new(idx: u32) -> Self {
          return Self(idx);
        }

        pub fn idx(&self) -> usize {
          return self.0 as usize;
        }
      }
    )*
  };
}
