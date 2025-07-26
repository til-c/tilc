#[derive(Debug)]
pub enum PrimTy {
  Int(IntTy),
  Uint(UintTy),
  Float(FloatTy),
  Bool,
  Str,
  Char,
}


#[derive(Debug)]
pub enum IntTy {
  I16,
  I32,
  I64,
  I128,
}

#[derive(Debug)]
pub enum UintTy {
  U16,
  U32,
  U64,
  U128,
}

#[derive(Debug)]
pub enum FloatTy {
  F32,
  F64,
}
