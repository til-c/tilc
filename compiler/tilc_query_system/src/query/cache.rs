use std::{collections::HashMap, fmt::Debug, hash::Hash, sync::OnceLock};

use indexmap::IndexMap;
use parking_lot::RwLock;
use tilc_span::{DefId, DefIdx, LOCAL_SANDYQ};


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
impl<V> QueryCache for UnitCache<V>
where
  V: Copy,
{
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
    return Self(Default::default());
  }
}


#[derive(Debug)]
pub struct DefaultCache<K, V> {
  map: RwLock<HashMap<K, V>>,
}
impl<K, V> Default for DefaultCache<K, V> {
  fn default() -> Self {
    return Self {
      map: RwLock::new(Default::default()),
    };
  }
}
impl<K, V> QueryCache for DefaultCache<K, V>
where
  K: Debug + Copy + Hash + Eq,
  V: Copy,
{
  type Key = K;
  type Value = V;


  fn lookup(&self, key: &Self::Key) -> Option<Self::Value> {
    return self.map.read().get(key).copied();
  }
  fn compute(&self, key: Self::Key, value: Self::Value) {
    self.map.write().insert(key, value);
  }
}

#[derive(Debug)]
pub struct DefIdCache<V> {
  local: RwLock<IndexMap<DefIdx, V>>,
  foreign: DefaultCache<DefId, V>,
}
impl<V> Default for DefIdCache<V> {
  fn default() -> Self {
    return Self {
      local: RwLock::new(IndexMap::new()),
      foreign: Default::default(),
    };
  }
}
impl<V> QueryCache for DefIdCache<V>
where
  V: Copy,
{
  type Key = DefId;
  type Value = V;


  fn lookup(&self, key: &Self::Key) -> Option<Self::Value> {
    if key.sandyq_idx == LOCAL_SANDYQ {
      return self.local.read().get(&key.def_idx).copied();
    } else {
      return self.foreign.lookup(key);
    };
  }
  fn compute(&self, key: Self::Key, value: Self::Value) {
    if key.sandyq_idx == LOCAL_SANDYQ {
      self.local.write().insert(key.def_idx, value);
    } else {
      self.foreign.compute(key, value);
    };
  }
}
