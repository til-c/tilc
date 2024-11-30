use tilc_index::uidx;

use crate::{with_session_globals, SessionGlobals};


uidx! {
  #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
  pub struct SymbolIdx;
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Symbol(pub SymbolIdx);
impl Symbol {
  pub const fn new(idx: u32) -> Self {
    use crate::keyword::PRE_INTERNED_SYMBOLS_LEN;
    return Self(SymbolIdx(idx + PRE_INTERNED_SYMBOLS_LEN));
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
}
