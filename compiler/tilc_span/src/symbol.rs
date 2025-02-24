use tilc_index::uidx;

use crate::{kw, with_session_globals, SessionGlobals, Span};


uidx! {
  #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub struct SymbolIdx;
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol(pub SymbolIdx);
impl Symbol {
  pub const fn new(idx: u32) -> Self {
    return Self(SymbolIdx(idx));
  }


  /// For symbols! macro use only
  pub(super) const fn new_m(idx: u32) -> Self {
    return Self(SymbolIdx(idx));
  }

  pub fn idx(&self) -> usize {
    return self.0.idx();
  }


  pub fn intern(string: &str) -> Self {
    return with_session_globals(|session_globals: &SessionGlobals| {
      return session_globals.symbol_interner.intern(string);
    });
  }

  pub fn is_reserved(self) -> bool {
    return self >= kw::As && self <= kw::Use;
  }
}
impl AsRef<Self> for Symbol {
  fn as_ref(&self) -> &Self {
    return &self;
  }
}


#[derive(Clone, Copy, Debug)]
pub struct Identifier {
  pub symbol: Symbol,
  pub span: Span,
}
impl Identifier {
  pub const EMPTY: Self = Self {
    symbol: Symbol(SymbolIdx::EMPTY),
    span: Span::EMPTY,
  };

  pub fn is_reserved(&self) -> bool {
    return self.symbol.is_reserved();
  }
}
