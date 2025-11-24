use std::collections::HashMap;

use tilc_ast::NodeIdx;
use tilc_span::LocalDefIdx;

#[derive(Debug)]
pub struct ResolverAstLowering {
  pub next_node_idx: NodeIdx,
  pub node_idx_to_def_idx: HashMap<NodeIdx, LocalDefIdx>,
}
