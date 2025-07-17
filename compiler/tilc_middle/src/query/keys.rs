use tilc_query_system::{DefIdCache, UnitCache};
use tilc_span::DefId;


pub trait Key: Sized {
  type Cache<V>;
}

impl Key for () {
  type Cache<V> = UnitCache<V>;
}
impl Key for DefId {
  type Cache<V> = DefIdCache<V>;
}
