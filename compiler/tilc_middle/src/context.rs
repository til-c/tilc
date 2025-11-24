use std::{marker::PhantomData, ops::Deref, sync::OnceLock};

use tilc_session::Session;
use tilc_span::Span;

use crate::{Arena, QuerySystem};

#[derive(Debug)]
pub struct GlobalCtxt<'ctxt> {
  session: &'ctxt Session,
  pub arena: &'ctxt Arena,

  pub query_system: QuerySystem<'ctxt>,
}
impl<'ctxt> GlobalCtxt<'ctxt> {
  pub fn enter<F, R>(&'ctxt self, f: F) -> R
  where
    F: FnOnce(TyCtxt<'ctxt>) -> R, {
    let tcx = TyCtxt { gcx: self };
    f(tcx)
  }
}

#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct TyCtxt<'ctxt> {
  gcx: &'ctxt GlobalCtxt<'ctxt>,
}
impl<'ctxt> TyCtxt<'ctxt> {
  pub fn create_global_ctxt<F, R>(
    gcx_cell: &'ctxt OnceLock<GlobalCtxt<'ctxt>>,

    session: &'ctxt Session,
    arena: &'ctxt Arena,
    query_system: QuerySystem<'ctxt>,
    // sandyq_id: SandyqId,
    f: F,
  ) -> R
  where
    F: FnOnce(TyCtxt<'ctxt>) -> R, {
    // let definitions: RwLock<IndexVec<DefIdx, DefKind>> = Default::default();
    // definitions.write().push(DefKind::Korpe);

    gcx_cell
      .get_or_init(|| GlobalCtxt {
        session,
        arena,
        query_system,
        // sandyq_id,

        // definitions,
      })
      .enter(f)
  }

  pub(crate) fn at(self, span: Span) -> TyCtxtAt<'ctxt> {
    TyCtxtAt { tcx: self, span }
  }

  pub fn feed<KEY>(self, key: KEY) -> TyCtxtFeed<'ctxt, KEY>
  where
    KEY: Copy, {
    TyCtxtFeed::<'ctxt, KEY> { tcx: self, key }
  }
}
impl<'ctxt> Deref for TyCtxt<'ctxt> {
  type Target = GlobalCtxt<'ctxt>;

  fn deref(&self) -> &Self::Target {
    self.gcx
  }
}

#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct TyCtxtFeed<'ctxt, KEY>
where
  KEY: Copy, {
  pub tcx: TyCtxt<'ctxt>,
  key: KEY,
}
impl<'ctxt, KEY> TyCtxtFeed<'ctxt, KEY>
where
  KEY: Copy,
{
  #[inline(always)]
  pub const fn downgrade(self) -> Feed<'ctxt, KEY> {
    Feed {
      tcx: PhantomData,
      key: self.key,
    }
  }

  #[inline(always)]
  pub const fn key(&self) -> KEY {
    self.key
  }
}

#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct Feed<'ctxt, KEY>
where
  KEY: Copy, {
  pub tcx: PhantomData<TyCtxt<'ctxt>>,
  key: KEY,
}
impl<'ctxt, KEY> Feed<'ctxt, KEY>
where
  KEY: Copy,
{
  #[inline(always)]
  pub const fn upgrade(self, tcx: TyCtxt<'ctxt>) -> TyCtxtFeed<'ctxt, KEY> {
    TyCtxtFeed { tcx, key: self.key }
  }

  #[inline(always)]
  pub const fn key(&self) -> KEY {
    self.key
  }
}

pub struct TyCtxtAt<'ctxt> {
  pub tcx: TyCtxt<'ctxt>,
  span: Span,
}
impl<'ctxt> Deref for TyCtxtAt<'ctxt> {
  type Target = TyCtxt<'ctxt>;

  fn deref(&self) -> &Self::Target {
    &self.tcx
  }
}
