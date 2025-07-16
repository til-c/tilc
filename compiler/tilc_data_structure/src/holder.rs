use parking_lot::{MappedRwLockReadGuard, RwLock, RwLockReadGuard};

#[derive(Debug)]
pub struct Holder<T> {
  value: RwLock<Option<T>>,
}
impl<T> Holder<T> {
  pub fn new(value: T) -> Self {
    return Self {
      value: RwLock::new(Some(value)),
    };
  }

  pub fn look(&self) -> MappedRwLockReadGuard<'_, T> {
    let borrow = self.value.read();

    if borrow.is_none() {
      panic!("Trying to look at empty value");
    };

    return RwLockReadGuard::map(borrow, |v| v.as_ref().unwrap());
  }
  pub fn steal(&self) -> T {
    let value = &mut *self.value.write();
    let value = value.take();
    return value.unwrap();
  }
  pub fn get_mut(&mut self) -> &mut T {
    return self
      .value
      .get_mut()
      .as_mut()
      .expect("Trying to get empty value");
  }
}
