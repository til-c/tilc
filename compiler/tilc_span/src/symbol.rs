use tilc_index::uidx;

uidx!(SymbolIdx);


pub struct Symbol(SymbolIdx);
impl Symbol {
  pub fn new(idx: u32) -> Self {
    return Self(SymbolIdx(idx));
  }

  pub fn idx(&self) -> usize {
    return self.0.idx();
  }
}
