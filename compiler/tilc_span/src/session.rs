use std::rc::Rc;

use crate::{Interner, SourceMap};

pub struct SessionGlobals {
  pub(crate) symbol_interner: Interner,

  source_map: Rc<SourceMap>,
}
impl SessionGlobals {
  pub(crate) fn new() -> Self {
    Self {
      symbol_interner: Interner::with_prefilled(),
      source_map: Rc::new(SourceMap::new()),
    }
  }
  pub fn source_map(&self) -> Rc<SourceMap> {
    self.source_map.clone()
  }
}
