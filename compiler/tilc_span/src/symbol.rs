use tilc_macro::uidx;

uidx! {
  #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
  pub struct SymbolIdx {}
}


#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Symbol(SymbolIdx);
impl Symbol {
  pub const fn new(idx: u32) -> Self {
    return Self(SymbolIdx(idx));
  }

  pub fn idx(&self) -> usize {
    return self.0.as_usize();
  }
}
