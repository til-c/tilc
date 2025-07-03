use tilc_span::{Edition, SourceMap};


#[derive(Debug)]
pub struct ParseSession {
  pub edition: Edition,

  source_map: SourceMap,
}
