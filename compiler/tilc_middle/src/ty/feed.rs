use std::marker::PhantomData;

use tilc_hir::DefKind;
use tilc_span::{LocalDefIdx, SANDYQ_DEF_IDX};

use crate::TyCtxt;


#[derive(Debug, Clone, Copy)]
pub struct TyCtxtFeed<'ctxt, KEY: Copy> {
  pub tcx: TyCtxt<'ctxt>,
  key: KEY,
}
impl<'ctxt, KEY: Copy> TyCtxtFeed<'ctxt, KEY> {
  #[inline(always)]
  pub fn downgrade(self) -> Feed<'ctxt, KEY> {
    return Feed {
      tcx: PhantomData,
      key: self.key,
    };
  }
  #[inline(always)]
  pub fn key(&self) -> KEY {
    return self.key;
  }
}


#[derive(Debug, Clone, Copy)]
pub struct Feed<'ctxt, KEY: Copy> {
  tcx: PhantomData<TyCtxt<'ctxt>>,
  key: KEY,
}
impl<'ctxt, KEY: Copy> Feed<'ctxt, KEY> {
  #[inline(always)]
  pub fn upgrade(self, tcx: TyCtxt<'ctxt>) -> TyCtxtFeed<'ctxt, KEY> {
    return TyCtxtFeed { tcx, key: self.key };
  }
  #[inline(always)]
  pub fn key(&self) -> KEY {
    return self.key;
  }
}


impl<'ctxt> TyCtxt<'ctxt> {
  pub fn unit_query_feed(self) -> TyCtxtFeed<'ctxt, ()> {
    return TyCtxtFeed { tcx: self, key: () };
  }

  pub fn local_sandyq_def_id_feed(self) -> TyCtxtFeed<'ctxt, LocalDefIdx> {
    return TyCtxtFeed {
      tcx: self,
      key: SANDYQ_DEF_IDX,
    };
  }

  pub fn create_def(self, def_kind: DefKind) -> TyCtxtFeed<'ctxt, LocalDefIdx> {
    let def_idx = self.definitions.write().push(def_kind);
    let key = LocalDefIdx { local: def_idx };

    let feed = TyCtxtFeed { tcx: self, key };
    dbg!(&self, &feed, def_idx, key, def_kind);
    feed.def_kind(def_kind);
    return feed;
  }
}
