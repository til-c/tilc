use std::{cell::RefCell, rc::Rc};

use crate::{
  ActualFileLoader, Interner, SourceFileHashAlgorithm, SourceMap,
  SourceMapFiles,
};

pub struct SessionGlobals {
  pub symbol_interner: Interner,

  pub source_map: Option<Rc<SourceMap>>,
}
impl SessionGlobals {
  pub fn new() -> Self {
    return Self {
      symbol_interner: Interner::with_prefilled(),

      source_map: Some(Rc::new(SourceMap {
        file_loader: Box::new(ActualFileLoader),
        files: RefCell::new(SourceMapFiles {
          ..Default::default()
        }),

        _hash_algorithm: SourceFileHashAlgorithm::default(),
      })),
    };
  }
}
