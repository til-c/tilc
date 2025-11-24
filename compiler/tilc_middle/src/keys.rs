use crate::UnitCache;

pub trait Key: Sized {
  type Cache<T: Clone>;
}

impl Key for () {
  type Cache<T: Clone> = UnitCache<T>;
}
