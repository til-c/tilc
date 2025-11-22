use tilc_macros::uidx;

use crate::{Span, kw, sym, with_session_globals};

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
    Self(SymbolIdx::from_u32(idx))
  }
  pub fn idx(&self) -> usize {
    self.0.as_usize()
  }

  pub fn intern(str: &str) -> Self {
    with_session_globals(|session_globals| session_globals.symbol_interner.intern(str))
  }

  fn is_reserved(self) -> bool {
    self >= kw::Let && self <= kw::Mut
  }
}

#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct Ident {
  pub name: Symbol,
  pub span: Span,
}
impl Ident {
  pub const DUMMY: Self = Self {
    name: sym::dummy,
    span: Span::EMPTY,
  };

  pub fn is_reserved(&self) -> bool {
    self.name.is_reserved()
  }
  pub fn is_path_segment_ident(&self) -> bool {
    return matches!(self.name, kw::Sandyq | kw::Super | kw::SelfValue);
  }
}
