use std::rc::Rc;

use indexmap::IndexMap;
use parking_lot::RwLock;
use tilc_ast::AttrIdxGen;
use tilc_span::{Edition, SourceMap, Span, Symbol};


#[derive(Debug)]
pub struct ParseSession {
  pub edition: Edition,
  pub symbol_repo: SymbolRepo,

  pub source_map: Rc<SourceMap>,

  pub attr_idx_gen: AttrIdxGen,
}
impl ParseSession {
  pub fn new(source_map: Rc<SourceMap>) -> Self {
    return Self {
      edition: Default::default(),
      symbol_repo: Default::default(),

      source_map,

      attr_idx_gen: AttrIdxGen::new(),
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
