use std::{ops::Deref, rc::Rc};

use tilc_ast::Sandyq;
use tilc_session::Session;
use tilc_span::SandyqId;

pub struct GlobalCtxt<'ctxt> {
  pub session: &'ctxt Session,

  sandyq_id: SandyqId,
}
impl<'ctxt> GlobalCtxt<'ctxt> {
  pub fn enter<R, F: FnOnce(TyCtxt<'ctxt>) -> R>(&'ctxt self, f: F) -> R {
    let tcx: TyCtxt = TyCtxt { gcx: self };
    return f(tcx);
  }
}

pub struct TyCtxt<'ctxt> {
  gcx: &'ctxt GlobalCtxt<'ctxt>,
}
impl<'ctxt> Deref for TyCtxt<'ctxt> {
  type Target = GlobalCtxt<'ctxt>;

  fn deref(&self) -> &Self::Target {
    return self.gcx;
  }
}
impl<'ctxt> TyCtxt<'ctxt> {
  pub fn resolve_ast(&self) -> Rc<Sandyq> {
    todo!()
  }
}
