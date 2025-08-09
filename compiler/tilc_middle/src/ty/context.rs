use std::{ops::Deref, sync::OnceLock};

use indexvec::IndexVec;
use parking_lot::RwLock;

use tilc_hir::DefKind;
use tilc_session::Session;
use tilc_span::{DefIdx, SandyqId};

use crate::{Arena, QuerySystem};


// TODO: deal with field visibilities
#[derive(Debug)]
pub struct GlobalCtxt<'ctxt> {
  pub session: &'ctxt Session,

  pub arena: &'ctxt Arena,
  pub query_system: QuerySystem<'ctxt>,

  sandyq_id: SandyqId,

  pub definitions: RwLock<IndexVec<DefIdx, DefKind>>,
}
impl<'ctxt> GlobalCtxt<'ctxt> {
  fn enter<R, F: FnOnce(TyCtxt<'ctxt>) -> R>(&'ctxt self, f: F) -> R {
    let tcx = TyCtxt { gcx: self };
    return f(tcx);
  }
}

#[derive(Debug, Clone, Copy)]
pub struct TyCtxt<'ctxt> {
  gcx: &'ctxt GlobalCtxt<'ctxt>,
}
impl<'ctxt> TyCtxt<'ctxt> {
  pub fn create_global_ctxt<T, F: FnOnce(TyCtxt<'ctxt>) -> T>(
    gcx_cell: &'ctxt OnceLock<GlobalCtxt<'ctxt>>,
    session: &'ctxt Session,
    arena: &'ctxt Arena,
    query_system: QuerySystem<'ctxt>,
    sandyq_id: SandyqId,
    f: F,
  ) -> T {
    let definitions: RwLock<IndexVec<DefIdx, DefKind>> = Default::default();
    definitions.write().push(DefKind::Korpe);

    return gcx_cell
      .get_or_init(|| GlobalCtxt {
        session,

        arena,
        query_system,

        sandyq_id,

        definitions,
      })
      .enter(f);
  }
}
impl<'ctxt> Deref for TyCtxt<'ctxt> {
  type Target = GlobalCtxt<'ctxt>;

  fn deref(&self) -> &Self::Target {
    return self.gcx;
  }
}
