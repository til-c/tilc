use std::sync::RwLock;

use indexmap::IndexMap;
use tilc_span::{Edition, Span, Symbol};


pub struct SymbolRepo {
  pub symbols: RwLock<IndexMap<Symbol, Span>>,
}
impl SymbolRepo {
  pub fn new() -> Self {
    return Self {
      symbols: RwLock::new(IndexMap::new()),
    };
  }


  pub fn insert(&self, symbol: Symbol, span: Span) {
    debug_assert!(!self.symbols.is_poisoned());
    self.symbols.write().unwrap().entry(symbol).or_insert(span);
  }
}
pub struct ParseSession {
  pub edition: Edition,

  pub symbol_repo: SymbolRepo,
}
