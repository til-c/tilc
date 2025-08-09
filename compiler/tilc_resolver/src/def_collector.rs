use tilc_ast::{ItemKind, NodeIdx, Walker, walk_item};
use tilc_hir::DefKind;
use tilc_span::LocalDefIdx;

use crate::Resolver;

pub(crate) struct DefCollector<'a, 'ctxt, 'ra> {
  pub(crate) resolver: &'a mut Resolver<'ctxt, 'ra>,
}
impl<'a, 'ctxt, 'ra> DefCollector<'a, 'ctxt, 'ra> {
  fn create_def(
    &mut self,
    node_idx: NodeIdx,
    def_kind: DefKind,
  ) -> LocalDefIdx {
    let feed = self.resolver.create_def(node_idx, def_kind);
    return feed.key();
  }
}
impl<'a, 'ctxt, 'ra> Walker for DefCollector<'a, 'ctxt, 'ra> {
  fn walk_item<'ast>(&mut self, item: &'ast tilc_ast::Item) {
    dbg!("emm", &item);
    let def_kind = match &item.kind {
      ItemKind::Korpe(..) => DefKind::Korpe,
      ItemKind::Fn(..) => DefKind::Fn,

      _ => todo!(),
    };

    // TODO: Walk impl item with parent def_idx from create_def
    self.create_def(item.idx, def_kind);

    walk_item(self, item);
  }
}
