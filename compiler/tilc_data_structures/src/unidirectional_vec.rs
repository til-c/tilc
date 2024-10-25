use std::ops::Deref;


pub struct UnidirectionalVec<T>(Vec<T>);
impl<T> UnidirectionalVec<T> {
  pub fn push(&mut self, item: T) {
    self.0.push(item);
  }
}
impl<T> Default for UnidirectionalVec<T> {
  fn default() -> Self {
    return Self(Vec::new());
  }
}
impl<T> Deref for UnidirectionalVec<T> {
  type Target = Vec<T>;


  fn deref(&self) -> &Self::Target {
    return &self.0;
  }
}
