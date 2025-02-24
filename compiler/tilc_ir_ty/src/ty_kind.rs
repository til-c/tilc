use crate::Interner;


/// Primitive type kinds
pub enum TyKind<I: Interner> {
  Char,

  Int(IntTy),
  Uint(UintTy),

  Float(FloatTy),

  Str,

  Array,
  Slice,
  Tuple,

  Never,

  Infer,

  Error(I::ErrorGuaranteed),
}

pub enum IntTy {
  Isize,
  I8,
  I16,
  I32,
  I64,
  I128,
}
pub enum UintTy {
  Usize,
  U8,
  U16,
  U32,
  U64,
  U128,
}
pub enum FloatTy {
  F32,
  F64,
  F128,
}
