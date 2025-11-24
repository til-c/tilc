use parking_lot::RwLock;

#[derive(Debug)]
pub struct Holder<T> {
  value: RwLock<Option<T>>,
}
impl<T> Holder<T> {
  pub const fn new(value: T) -> Self {
    Self {
      value: RwLock::new(Some(value)),
    }
  }

  pub fn steal(&self) -> T {
    let value = &mut *self.value.write();
    let value = value.take();
    value.unwrap()
  }
}
