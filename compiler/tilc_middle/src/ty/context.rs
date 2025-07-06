use std::{ops::Deref, sync::OnceLock};

use tilc_session::Session;
use tilc_span::SandyqId;


// TODO: deal with field visibilities
#[derive(Debug)]
pub struct GlobalCtxt<'ctxt> {
  pub session: &'ctxt Session,

  sandyq_id: SandyqId,
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
  pub fn create_and_enter_global_ctxt<T, F: FnOnce(TyCtxt<'ctxt>) -> T>(
    gcx_cell: &'ctxt OnceLock<GlobalCtxt<'ctxt>>,
    session: &'ctxt Session,
    sandyq_id: SandyqId,
    f: F,
  ) -> T {
    return gcx_cell
      .get_or_init(|| GlobalCtxt { session, sandyq_id })
      .enter(f);
  }
}
impl<'ctxt> Deref for TyCtxt<'ctxt> {
  type Target = GlobalCtxt<'ctxt>;

  fn deref(&self) -> &Self::Target {
    return self.gcx;
  }
}
