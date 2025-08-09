use tilc_span::DefId;

use crate::PrimTy;


#[derive(Debug)]
pub enum Res {
  Local,
  PrimTy(PrimTy),
  Def(DefKind, DefId),
}

#[derive(Debug, Clone, Copy)]
pub enum DefKind {
  Korpe,
  Use,

  Struct,
  Enum,
  Union,
  Trait,

  Fn,
}
