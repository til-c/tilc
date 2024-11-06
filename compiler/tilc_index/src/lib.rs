#[macro_export]
macro_rules! uidx {
  ($name:ident$(;),* $($derives:ident),*) => {
    #[derive($($derives),*)]
    pub struct $name(u32);

    impl $name {
      pub const fn new(idx: u32) -> Self {
        return Self(idx);
      }

      pub fn idx(&self) -> usize {
        return self.0 as usize;
      }
    }
  };
}
