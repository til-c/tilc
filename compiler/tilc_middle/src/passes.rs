use std::sync::Arc;

use tilc_ast::Sandyq;
use tilc_data_structures::Holder;

use crate::{ResolverAstLowering, TyCtxt};

pub(crate) fn resolver_for_lowering_raw<'tcxt>(
  tcx: TyCtxt<'tcxt>,
  _: (),
) -> &'tcxt (ResolverAstLowering, Holder<Arc<Sandyq>>) {
  let mut sandyq = tcx.sandyq_for_resolver(());
  todo!("resolver");
}
