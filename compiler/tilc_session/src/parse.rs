use std::rc::Rc;

use indexmap::IndexMap;
use parking_lot::RwLock;
use tilc_ast::{AttrIdx, AttrIdxGen};
use tilc_span::{Edition, SourceMap, Span, Symbol};

#[derive(Debug)]
pub struct ParseSession {
  edition: Edition,
  pub symbol_repo: SymbolRepo,

  source_map: Rc<SourceMap>,

  attr_idx_gen: AttrIdxGen,
}
impl ParseSession {
  pub fn new(source_map: Rc<SourceMap>) -> Self {
    Self {
      edition: Default::default(),
      symbol_repo: SymbolRepo(Default::default()),

      source_map,

      attr_idx_gen: AttrIdxGen::new(),
    }
  }

  pub fn source_map(&self) -> Rc<SourceMap> {
    self.source_map.clone()
  }
  pub fn make_attr_idx(&self) -> AttrIdx {
    return self.attr_idx_gen.make_attr_idx();
  }
}

#[derive(Debug)]
pub struct SymbolRepo(RwLock<IndexMap<Symbol, Span>>);
impl SymbolRepo {
  pub fn insert(&self, symbol: Symbol, span: Span) {
    debug_assert!(!self.0.is_locked());
    self.0.write().entry(symbol).or_insert_with(|| span);
  }
}
