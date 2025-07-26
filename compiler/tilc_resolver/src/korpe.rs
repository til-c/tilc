use std::{fmt::Debug, ops::Deref};

use tilc_data_structure::Interned;
use tilc_hir::DefKind;
use tilc_span::{DefId, Span, Symbol};


#[derive(Clone, Copy)]
pub(crate) struct Korpe<'ra>(pub Interned<'ra, KorpeData<'ra>>);
impl<'ra> Korpe<'ra> {
  pub(crate) fn opt_def_id(&self) -> Option<DefId> {
    return match self.kind {
      KorpeKind::Def(_, def_id, _) => Some(def_id),
      KorpeKind::Block => None,
    };
  }
}
impl<'ra> Deref for Korpe<'ra> {
  type Target = KorpeData<'ra>;

  fn deref(&self) -> &Self::Target {
    return &self.0;
  }
}
impl<'ra> Debug for Korpe<'ra> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    return f.debug_tuple("Korpe").field(&self.0).finish();
  }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct KorpeData<'ra> {
  parent: Option<Korpe<'ra>>,
  kind: KorpeKind,

  span: Span,
}
impl<'ra> KorpeData<'ra> {
  pub fn new(parent: Option<Korpe<'ra>>, kind: KorpeKind, span: Span) -> Self {
    return Self { parent, kind, span };
  }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum KorpeKind {
  Block,
  Def(DefKind, DefId, Symbol),
}
