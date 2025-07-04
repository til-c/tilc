use std::rc::Rc;

use crate::{Interner, SourceMap};


pub struct SessionGlobals {
  pub symbol_interner: Interner,

  pub source_map: Option<Rc<SourceMap>>,
}
impl SessionGlobals {
  pub fn new() -> Self {
    return Self {
      // TODO: Create prefill function for keywords
      symbol_interner: Interner::with_prefilled(),

      source_map: Some(Rc::new(SourceMap {
        files: Default::default(),
      })),
    };
  }
}
