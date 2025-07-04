use tilc_macro::uidx;

use crate::{Span, kw, sym, with_session_globals};


uidx! {
  #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
  pub struct SymbolIdx {}
}


#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Symbol(SymbolIdx);
impl Symbol {
  pub const fn new(idx: u32) -> Self {
    return Self(SymbolIdx(idx));
  }

  pub fn idx(&self) -> usize {
    return self.0.as_usize();
  }


  pub fn intern(string: &str) -> Self {
    return with_session_globals(|session_globals| {
      return session_globals.symbol_interner.intern(string);
    });
  }

  pub fn is_reserved(self) -> bool {
    return self >= kw::As && self <= kw::Use;
  }
}

#[derive(Debug, Clone)]
pub struct Ident {
  pub name: Symbol,
  pub span: Span,
}
impl Ident {
  pub const DUMMY: Self = Self::new(sym::dummy, Span::EMPTY);


  pub const fn new(name: Symbol, span: Span) -> Self {
    return Self { name, span };
  }

  pub fn is_reserved(&self) -> bool {
    return self.name.is_reserved();
  }
  pub fn is_path_segment_ident(&self) -> bool {
    return matches!(self.name, kw::Crate | /* kw::Super | */ kw::SelfValue);
  }
}
