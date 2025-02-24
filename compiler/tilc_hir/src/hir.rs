use tilc_span::{ErrorGuaranteed, Identifier, Span};

use crate::HirId;


#[derive(Debug)]
pub struct Ty<'hir> {
  pub hir_id: HirId,
  pub kind: TyKind<'hir>,
  pub span: Span,
}

#[derive(Debug)]
pub enum TyKind<'a> {
  Path(&'a [Ty<'a>]),

  Array,
  Slice,
  Tuple,

  Never,

  Infer,

  Error(ErrorGuaranteed),
}

#[derive(Debug)]
pub enum PrimTy {
  Bool,
  Char,

  Int,
  Uint,
  Float,

  Unit,
}

#[derive(Debug)]
pub enum Resolution {
  Definition,

  PrimitiveTy(PrimTy),
}

#[derive(Debug)]
pub struct PathSegment {
  pub(crate) ident: Identifier,
  pub(crate) span: Span,
}
#[derive(Debug)]
pub struct Path<'a> {
  pub(crate) span: Span,
  pub(crate) res: Resolution,
  pub(crate) segments: &'a [PathSegment],
}
