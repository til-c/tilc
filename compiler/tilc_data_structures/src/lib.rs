use std::ops::Deref;

pub struct UnidirectionalVector<T>(Vec<T>);
impl<T> UnidirectionalVector<T> {
  pub fn push(&mut self, item: T) {
    self.0.push(item);
  }
}
impl<T> Default for UnidirectionalVector<T> {
  fn default() -> Self {
    return Self(Vec::<T>::new());
  }
}
impl<T> Deref for UnidirectionalVector<T> {
  type Target = Vec<T>;

  fn deref(&self) -> &Self::Target {
    return &self.0;
  }
}
