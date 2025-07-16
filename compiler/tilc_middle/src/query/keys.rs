use tilc_query_system::UnitCache;


pub trait Key: Sized {
  type Cache<V>;
}

impl Key for () {
  type Cache<V> = UnitCache<V>;
}
