use std::fmt::Debug;

use crate::{Providers, QueryCache, QueryCaches, TyCtxt};

pub fn get_query<'ctxt, Cache>(
  tcx: TyCtxt<'ctxt>,
  query_cache: &Cache,
  execute_query: fn(TyCtxt<'ctxt>, Cache::Key) -> Cache::Value,
  key: Cache::Key,
) -> Cache::Value
where
  Cache: QueryCache, {
  let key = key.into_query_key();

  match query_cache.lookup(&key) {
    Some(value) => value,
    None => execute_query(tcx, key),
  }
}

pub trait IntoQueryKey<P> {
  fn into_query_key(self) -> P;
}
impl<P> IntoQueryKey<P> for P {
  fn into_query_key(self) -> P {
    self
  }
}

pub struct QuerySystem<'ctxt> {
  pub caches: QueryCaches<'ctxt>,
  pub fns: QueryFns,
}
impl<'ctxt> Debug for QuerySystem<'ctxt> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("QuerySystem<'ctxt>")
      .field("fns", &self.fns)
      .finish_non_exhaustive()
  }
}
#[derive(Debug)]
pub struct QueryFns {
  pub local_providers: Providers,
}

macro_rules! define_callbacks {
  ($(
    $(#[$attrs:meta])*
    [$(($modifiers:tt))*] fn $name:ident($($k:tt)*) -> $v:ty,
  )*) => {
    use super::*;

    pub mod queries {
      use super::*;

      $(pub mod $name {
        use super::*;

        pub type Key<'ctxt> = $($k)*;
        pub type Value<'ctxt> = $v;

        pub type LocalKey<'ctxt> = $($k)*;
        pub type ProvidedValue<'ctxt> = $v;

        pub type Storage<'ctxt> = <$($k)* as keys::Key>::Cache<$v>;
      })*

      #[derive(Debug)]
      pub struct Providers {$(
        pub $name: for<'ctxt> fn(TyCtxt<'ctxt>, queries::$name::LocalKey<'ctxt>) -> queries::$name::ProvidedValue<'ctxt>,
      )*}
      impl Providers {
        pub const fn new() -> Self {
          Self {$(
            $name: |_, key| default_query(stringify!($name), &key),
          )*}
        }
      }
      impl Clone for Providers {
        fn clone(&self) -> Self {
          *self
        }
      }
      impl Copy for Providers {}
    }

    #[derive(Default)]
    pub struct QueryCaches<'ctxt> {$(
      pub $name: queries::$name::Storage<'ctxt>,
    )*}

    impl<'ctxt> TyCtxt<'ctxt> {$(
      #[inline(always)]
      pub fn $name(self, key: $($k)*) -> $v {
        self.at(tilc_span::Span::EMPTY).$name(key)
      }
    )*}
    impl<'ctxt> TyCtxtAt<'ctxt> {$(
      pub fn $name(self, key: $($k)*) -> $v {
        get_query(self.tcx, &self.query_system.caches.$name, self.query_system.fns.local_providers.$name, key)
      }
    )*}
  };
}

macro_rules! define_feedables {
  ($(
    $(#[$attrs:meta])*
    [$(($modifiers:tt))*] fn $name:ident($($k:tt)*) -> $v:ty,
  )*) => {$(
    impl<'ctxt, KEY> TyCtxtFeed<'ctxt, KEY>
    where
    KEY: IntoQueryKey<$($k)*> + Copy, {
      $(#[$attrs])*
      pub fn $name(self, value: queries::$name::ProvidedValue<'ctxt>) {
        let key = self.key().into_query_key();
        let cache = &self.tcx.query_system.caches.$name;

        match cache.lookup(&key) {
          Some(_) => panic!("Value is already been computed"),
          None => cache.compute(key, value),
        };
      }
    }
  )*};
}
