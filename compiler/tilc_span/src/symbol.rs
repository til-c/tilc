use tilc_index::uidx;

use crate::{with_session_globals, SessionGlobals};

uidx!(SymbolIdx; Clone, Copy, Debug, PartialEq, Eq, Hash);


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Symbol(SymbolIdx);
impl Symbol {
  pub const fn new(idx: u32) -> Self {
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
