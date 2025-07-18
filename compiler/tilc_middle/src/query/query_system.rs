use std::ops::Deref;

use tilc_query_system::{QueryCache, try_get_cache};
use tilc_span::{DefId, LocalDefIdx, Span};

use crate::{QueryCaches, TyCtxt, query::Providers};


pub fn query_get<'ctxt, Cache>(
  tcx: TyCtxt<'ctxt>,
  run_query: fn(TyCtxt<'ctxt>, Cache::Key) -> Cache::Value,
  query_cache: &Cache,
  key: Cache::Key,
) -> Cache::Value
where
  Cache: QueryCache, {
  let key = key.into_query_param();
  match try_get_cache(query_cache, &key) {
    Some(value) => return value,
    None => return run_query(tcx, key),
  };
}
impl<'ctxt> TyCtxt<'ctxt> {
  pub(crate) fn at(self, span: Span) -> TyCtxtAt<'ctxt> {
    return TyCtxtAt { tcx: self, span };
  }
}
pub struct TyCtxtAt<'ctxt> {
  pub tcx: TyCtxt<'ctxt>,
  pub span: Span,
}
impl<'ctxt> Deref for TyCtxtAt<'ctxt> {
  type Target = TyCtxt<'ctxt>;

  fn deref(&self) -> &Self::Target {
    return &self.tcx;
  }
}


#[derive(Debug)]
pub struct QuerySystem<'ctxt> {
  pub fns: QuerySystemFns,
  pub caches: QueryCaches<'ctxt>,
}
#[derive(Debug)]
pub struct QuerySystemFns {
  pub local_providers: Providers,
}


pub trait IntoQueryParam<P> {
  fn into_query_param(self) -> P;
}
impl<P> IntoQueryParam<P> for P {
  #[inline(always)]
  fn into_query_param(self) -> P {
    return self;
  }
}
impl IntoQueryParam<DefId> for LocalDefIdx {
  fn into_query_param(self) -> DefId {
    return self.to_def_id();
  }
}


macro_rules! query_helper_param_ty {
  (DefId) => { impl IntoQueryParam<DefId> };
  (LocalDefId) => { impl IntoQueryParam<LocalDefId> };
  ($K:ty) => { $K };
}
macro_rules! define_callbacks {
  ($(
    $(#[$attrs:meta])*
    [$($modifiers:tt)*] fn $name:ident($($k:tt)*) -> $v:ty,
  )*) => {
    use tilc_span::Span;

    use crate::*;


    pub mod queries {
      pub use super::*;

      $(pub mod $name {
        use super::super::*;


        pub type Key<'ctxt> = $($k)*;
        pub type Value<'ctxt> = $v;

        pub type LocalKey<'ctxt> = $($k)*;
        pub type ProvidedValue<'ctxt> = $v;

        pub type Storage<'ctxt> = <$($k)* as keys::Key>::Cache<$v>;
      })*
    }

    #[derive(Debug, Clone, Copy)]
      pub struct Providers {
        $(pub $name: for<'ctxt> fn(TyCtxt<'ctxt>, queries::$name::LocalKey<'ctxt>) -> queries::$name::ProvidedValue<'ctxt>,)*
      }
      impl Default for Providers {
        fn default() -> Self {
          return Self {
            $($name: |_, key| $crate::query::default_query(stringify!($name), &key)),*
          };
        }
      }

    impl<'ctxt> TyCtxt<'ctxt> {$(
      $(#[$attrs])*
      #[inline(always)]
      pub fn $name(self, key: query_helper_param_ty!($($k)*)) ->$v {
        return self.at(Span::EMPTY).$name(key);
      }
    )*}
    impl<'ctxt> TyCtxtAt<'ctxt> {$(
      pub fn $name(self, key: query_helper_param_ty!($($k)*)) -> $v {
        let key = key.into_query_param();
        dbg!(self::descs::$name(self.tcx, key));
        return query_get(self.tcx, self.query_system.fns.local_providers.$name, &self.query_system.caches.$name, key);
      }
    )*}

    #[derive(Debug)]
    pub struct QueryEngine {$(
      pub $name: for<'ctxt> fn (TyCtxt<'ctxt>, ::tilc_span::Span) -> Option<$v>,
    )*}
    #[derive(Debug, Default)]
    pub struct QueryCaches<'ctxt> {$(
      pub $name: queries::$name::Storage<'ctxt>,
    )*}
  };
}

macro_rules! define_queries {
  ($(
    $(#[$attrs:meta])*
    [$($modifiers:tt)*] fn $name:ident($($k:tt)*) -> $v:ty,
  )*) => {$(
    impl<'ctxt, K: IntoQueryParam<$($k)*> + Copy> TyCtxtFeed<'ctxt, K> {
      $(#[$attrs])*
      pub fn $name(self, value: queries::$name::ProvidedValue<'ctxt>) {
        let key = self.key().into_query_param();

        let tcx = self.tcx;
        let cache = &tcx.query_system.caches.$name;

        match try_get_cache(cache, &key) {
          Some(value) => {
            panic!("This value: {:?} is already cached", value);
          }
          None => {
            cache.compute(key, value);
          }
        };
      }
    }
  )*};
}
