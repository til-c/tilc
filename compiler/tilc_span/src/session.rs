use std::rc::Rc;

use crate::SourceMap;


pub struct SessionGlobals {
  pub source_map: Option<Rc<SourceMap>>,
}
impl SessionGlobals {
  pub fn new() -> Self {
    return Self {
      source_map: Some(Rc::new(SourceMap {
        files: Default::default(),
      })),
    };
  }
}
