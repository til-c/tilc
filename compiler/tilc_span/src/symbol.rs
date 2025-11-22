use tilc_macros::uidx;

use crate::with_session_globals;

uidx! {
  struct SymbolIdx {}
}

#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
#[derive(Hash)]
pub struct Symbol(SymbolIdx);
impl Symbol {
  pub const fn new(idx: u32) -> Self {
    return Self(SymbolIdx::from_u32(idx));
  }
  pub fn idx(&self) -> usize {
    return self.0.as_usize();
  }

  pub fn intern(str: &str) -> Self {
    return with_session_globals(|session_globals| session_globals.symbol_interner.intern(str));
  }
}
