use std::{
  collections::{HashMap, hash_map::Entry},
  fmt::Debug,
  hash::Hash,
  sync::OnceLock,
};

use parking_lot::RwLock;

pub trait QueryCache
where
  Self: Default, {
  type Key: Debug + Copy + PartialEq + Eq + Hash;
  type Value: Clone;

  fn lookup(&self, key: &Self::Key) -> Option<Self::Value>;
  fn compute(&self, key: Self::Key, value: Self::Value);
}

#[derive(Debug)]
pub struct UnitCache<T>(OnceLock<T>);
impl<T> QueryCache for UnitCache<T>
where
  T: Clone,
{
  type Key = ();
  type Value = T;

  fn lookup(&self, _: &Self::Key) -> Option<Self::Value> {
    self.0.get().cloned()
  }
  fn compute(&self, key: Self::Key, value: Self::Value) {
    if let Some(_) = self.lookup(&key) {
      panic!("Value should not be computed twice");
    };
    self.0.set(value).ok().unwrap();
  }
}
impl<T> Default for UnitCache<T>
where
  T: Clone,
{
  fn default() -> Self {
    Self(OnceLock::new())
  }
}

#[derive(Debug)]
pub struct DefaultCahce<K, V>
where
  K: Debug + Copy + PartialEq + Eq + Hash,
  V: Clone, {
  map: RwLock<HashMap<K, V>>,
}
impl<K, V> QueryCache for DefaultCahce<K, V>
where
  K: Debug + Copy + PartialEq + Eq + Hash,
  V: Clone,
{
  type Key = K;
  type Value = V;

  fn lookup(&self, key: &Self::Key) -> Option<Self::Value> {
    let guard = self.map.read();
    guard.get(key).cloned()
  }
  fn compute(&self, key: Self::Key, value: Self::Value) {
    let mut guard = self.map.write();
    match guard.entry(key) {
      Entry::Vacant(entry) => entry.insert(value),
      _ => panic!("Value should not be computed twice"),
    };
  }
}
impl<K, V> Default for DefaultCahce<K, V>
where
  K: Debug + Copy + PartialEq + Eq + Hash,
  V: Clone,
{
  fn default() -> Self {
    Self {
      map: RwLock::new(HashMap::new()),
    }
  }
}
