#[macro_export]
macro_rules! uidx {
  ($name:ident) => {
    pub struct $name(u32);

    impl $name {
      pub fn idx(&self) -> usize {
        return self.0 as usize;
      }
    }
  };
}
