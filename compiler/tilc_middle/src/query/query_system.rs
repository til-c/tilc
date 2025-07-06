use std::ops::Deref;

use tilc_span::Span;

use crate::TyCtxt;
use crate::query::QueryEngine;


impl<'ctxt> TyCtxt<'ctxt> {
  fn at(self, span: Span) -> TyCtxtAt<'ctxt> {
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
pub struct QuerySystem {}
pub struct QuerySystemFns {
  pub query_engine: QueryEngine,
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
    use crate::TyCtxt;

    pub mod queries {$(
      pub mod $name {
        pub type Key<'ctxt> = $($k)*;
        pub type Value = $v;
      }
    )*}

    impl<'ctxt> TyCtxt<'ctxt> {$(
      $(#[$attrs])*
      #[inline(always)]
      pub fn $name(self, key: query_helper_param_ty!($($k)*)) ->$v {
        panic!("a");
      }
    )*}
    impl<'ctxt> TyCtxtAt<'ctxt> {$(
      pub fn $name(self, key: query_helper_param_ty!($($k)*)) -> $v {
        todo!();
      }
    )*}

    pub struct QueryEngine {$(
      pub $name: for<'ctxt> fn (TyCtxt<'ctxt>, ::tilc_span::Span) -> Option<$v>,
    )*}
  };
}
