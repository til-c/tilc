use std::{fmt::Debug, hash::Hash, sync::OnceLock};


pub fn try_get_cache<Cache>(
  cache: &Cache,
  key: &Cache::Key,
) -> Option<Cache::Value>
where
  Cache: QueryCache, {
  match cache.lookup(key) {
    Some(value) => return Some(value),
    None => return None,
  };
}

pub trait QueryCache: Sized {
  type Key: Debug + Copy + Hash + Eq;
  type Value: Copy;


  fn lookup(&self, key: &Self::Key) -> Option<Self::Value>;
  fn compute(&self, key: Self::Key, value: Self::Value);
}


#[derive(Debug)]
pub struct UnitCache<V>(OnceLock<V>);
impl<V: Copy> QueryCache for UnitCache<V> {
  type Key = ();
  type Value = V;


  #[inline(always)]
  fn lookup(&self, _: &Self::Key) -> Option<Self::Value> {
    return self.0.get().copied();
  }
  #[inline(always)]
  fn compute(&self, _: Self::Key, value: Self::Value) {
    self.0.set(value).ok().unwrap();
  }
}
impl<V> Default for UnitCache<V> {
  fn default() -> Self {
    return Self(OnceLock::default());
  }
}
