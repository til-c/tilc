use std::{rc::Rc, sync::RwLock};

use indexmap::IndexMap;
use tilc_errors::{DiagCtxt, DiagCtxtHandle};
use tilc_span::{Edition, SourceMap, Span, Symbol};


#[derive(Debug)]
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
#[derive(Debug)]
pub struct ParseSession {
  dcx: DiagCtxt,

  pub edition: Edition,

  pub symbol_repo: SymbolRepo,

  source_map: Rc<SourceMap>,
}
impl ParseSession {
  pub fn new(source_map: Rc<SourceMap>) -> Self {
    return Self {
      dcx: DiagCtxt::new(),

      edition: Edition::default(),
      symbol_repo: SymbolRepo::new(),

      source_map,
    };
  }

  pub fn source_map(&self) -> &SourceMap {
    return &self.source_map;
  }


  pub fn dcx(&self) -> DiagCtxtHandle {
    return self.dcx.handle();
  }
}
