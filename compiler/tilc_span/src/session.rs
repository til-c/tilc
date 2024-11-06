use std::rc::Rc;

use crate::{Interner, SourceMap};

pub struct SessionGlobals {
  pub symbol_interner: Interner,

  source_map: Option<Rc<SourceMap>>,
}
impl SessionGlobals {
  pub fn new() -> Self {
    return Self {
      symbol_interner: Interner::new(),

      source_map: None,
    };
  }
}
