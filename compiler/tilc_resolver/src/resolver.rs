use std::collections::HashMap;

use tilc_ast::{NodeIdx, SANDYQ_NODE_IDX};
use tilc_hir::DefKind;
use tilc_middle::{Feed, TyCtxt};
use tilc_span::LocalDefIdx;


#[derive(Debug)]
pub struct Resolver<'ctxt> {
  tcx: TyCtxt<'ctxt>,

  next_node_idx: NodeIdx,
  node_idx_to_def_idx: HashMap<NodeIdx, Feed<'ctxt, LocalDefIdx>>,
}
impl<'ctxt> Resolver<'ctxt> {
  pub fn new(tcx: TyCtxt<'ctxt>) -> Self {
    let mut node_idx_to_def_idx = HashMap::default();

    let sandyq_feed = tcx.local_sandyq_def_id_feed();
    sandyq_feed.def_kind(DefKind::Korpe);

    let sandyq_feed = sandyq_feed.downgrade();
    node_idx_to_def_idx.insert(SANDYQ_NODE_IDX, sandyq_feed);

    return Self {
      tcx,

      next_node_idx: SANDYQ_NODE_IDX,
      node_idx_to_def_idx,
    };
  }

  pub fn tcx(&self) -> TyCtxt<'ctxt> {
    return self.tcx;
  }

  pub fn next_node_idx(&mut self) -> NodeIdx {
    let id = self.next_node_idx;
    let next = id.as_u32().checked_add(1).expect("Too many NodeIds");
    self.next_node_idx = NodeIdx::from_u32(next);

    return id;
  }
}
