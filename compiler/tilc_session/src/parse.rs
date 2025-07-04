use std::rc::Rc;

use indexmap::IndexMap;
use parking_lot::RwLock;
use tilc_span::{Edition, SourceMap, Span, Symbol};


#[derive(Debug)]
pub struct ParseSession {
  pub edition: Edition,
  pub symbol_repo: SymbolRepo,

  source_map: Rc<SourceMap>,
}
impl ParseSession {
  pub fn new(source_map: Rc<SourceMap>) -> Self {
    return Self {
      edition: Edition::default(),
      symbol_repo: SymbolRepo::default(),

      source_map,
    };
  }
}

#[derive(Debug, Default)]
pub struct SymbolRepo {
  pub symbols: RwLock<IndexMap<Symbol, Span>>,
}
impl SymbolRepo {
  pub fn insert(&self, symbol: Symbol, span: Span) {
    debug_assert!(!self.symbols.is_locked());
    self.symbols.write().entry(symbol).or_insert_with(|| span);
  }
}
