use std::rc::Rc;

use tilc_span::{Edition, SourceMap};


#[derive(Debug)]
pub struct ParseSession {
  pub edition: Edition,

  source_map: Rc<SourceMap>,
}
impl ParseSession {
  pub fn new(source_map: Rc<SourceMap>) -> Self {
    return Self {
      edition: Edition::default(),

      source_map,
    };
  }
}
