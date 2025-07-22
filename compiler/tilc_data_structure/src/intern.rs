use std::ops::Deref;


#[derive(Debug, Clone, Copy)]
pub struct Interned<'a, T>(pub &'a T);
impl<'a, T> Interned<'a, T> {
  pub fn new(value: &'a T) -> Self {
    return Self(value);
  }
}
impl<'a, T> Deref for Interned<'a, T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    return self.0;
  }
}
