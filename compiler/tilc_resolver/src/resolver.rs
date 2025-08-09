use std::{cell::RefCell, collections::HashMap};

use indexmap::IndexMap;

use tilc_arena::TypedArena;
use tilc_ast::{
  Attribute, Item, NodeIdx, SANDYQ_NODE_IDX, Sandyq, Walker, walk_sandyq,
};
use tilc_data_structure::Interned;
use tilc_expand::{AstFragment, ResolverExpander};
use tilc_hir::{DefKind, Res};
use tilc_middle::{Feed, TyCtxt, TyCtxtFeed};
use tilc_span::{Ident, LocalDefIdx, SANDYQ_DEF_IDX, Span, sym};

use crate::{DefCollector, Korpe, KorpeData, KorpeKind};


#[derive(Debug, Default)]
pub struct ResolverArenas<'ra> {
  korpes: TypedArena<KorpeData<'ra>>,
  local_korpes: RefCell<Vec<Korpe<'ra>>>,
}
impl<'ra> ResolverArenas<'ra> {
  fn new_korpe(
    &'ra self,
    parent: Option<Korpe<'ra>>,
    kind: KorpeKind,
    span: Span,
  ) -> Korpe<'ra> {
    let korpe = Korpe(Interned::new(
      self.korpes.alloc(KorpeData::new(parent, kind, span)),
    ));
    let def_id = korpe.opt_def_id();
    if def_id.is_none_or(|def_id| def_id.is_local()) {
      self.local_korpes.borrow_mut().push(korpe);
    };

    return korpe;
  }
}


#[derive(Debug)]
pub struct Resolver<'ctxt, 'ra> {
  tcx: TyCtxt<'ctxt>,

  arenas: &'ra ResolverArenas<'ra>,
  root_korpe: Korpe<'ra>,

  next_node_idx: NodeIdx,
  node_idx_to_def_idx: HashMap<NodeIdx, Feed<'ctxt, LocalDefIdx>>,
}
impl<'ctxt, 'ra> Resolver<'ctxt, 'ra> {
  pub fn new(
    tcx: TyCtxt<'ctxt>,
    arenas: &'ra ResolverArenas<'ra>,
    sandyq_span: Span,
  ) -> Self {
    let root_def_id = SANDYQ_DEF_IDX.to_def_id();

    let root_korpe = arenas.new_korpe(
      None,
      KorpeKind::Def(DefKind::Korpe, root_def_id, sym::root_crate),
      sandyq_span,
    );

    let mut node_idx_to_def_idx: HashMap<NodeIdx, Feed<'ctxt, LocalDefIdx>> =
      Default::default();

    let sandyq_feed = tcx.local_sandyq_def_id_feed();
    sandyq_feed.def_kind(DefKind::Korpe);

    let sandyq_feed = sandyq_feed.downgrade();
    node_idx_to_def_idx.insert(SANDYQ_NODE_IDX, sandyq_feed);

    return Self {
      tcx,

      arenas,
      root_korpe,

      next_node_idx: SANDYQ_NODE_IDX,
      node_idx_to_def_idx,
    };
  }
  pub fn arenas() -> ResolverArenas<'ra> {
    return ResolverArenas::default();
  }

  pub fn tcx(&self) -> TyCtxt<'ctxt> {
    return self.tcx;
  }
  pub(crate) fn next_node_idx(&mut self) -> NodeIdx {
    let id = self.next_node_idx;
    let next = id.as_u32().checked_add(1).expect("Too many NodeIds");
    self.next_node_idx = NodeIdx::from_u32(next);

    return id;
  }

  // TODO: Parent and macro expansion specific definition
  pub(crate) fn create_def(
    &mut self,
    node_idx: NodeIdx,
    def_kind: DefKind,
  ) -> TyCtxtFeed<'ctxt, LocalDefIdx> {
    let feed = self.tcx.create_def(def_kind);
    if node_idx != NodeIdx::DUMMY {
      self.node_idx_to_def_idx.insert(node_idx, feed.downgrade());
    };


    return feed;
  }

  pub fn resolve_sandyq(&mut self, sandyq: &Sandyq) {
    let mut namespace_resolver = ResolveNamespace::new(self);
    walk_sandyq(&mut namespace_resolver, sandyq);
  }
}
impl<'ctxt, 'ra> ResolverExpander for Resolver<'ctxt, 'ra> {
  fn next_node_idx(&mut self) -> NodeIdx {
    return self.next_node_idx();
  }

  fn def_colletor<'a>(&mut self, ast_fragment: &'a AstFragment) {
    let mut def_collector = DefCollector { resolver: self };

    ast_fragment.walk_ast(&mut def_collector);
  }
}


#[derive(Debug)]
struct ResolveNamespace<'a, 'ctxt, 'ra> {
  resolver: &'a mut Resolver<'ctxt, 'ra>,

  type_ns: Vec<Scope>,
  value_ns: Vec<Scope>,
}
impl<'a, 'ctxt, 'ra> ResolveNamespace<'a, 'ctxt, 'ra> {
  fn new(resolver: &'a mut Resolver<'ctxt, 'ra>) -> Self {
    return Self {
      resolver,

      type_ns: Vec::new(),
      value_ns: Vec::new(),
    };
  }


  fn resolve_item<'ast>(&mut self, item: &'ast Item) {
    let def_kind = self.resolver.node_idx_to_def_idx.get(&item.idx).unwrap();
    dbg!(&def_kind);
  }
}
impl<'a, 'ctxt, 'ra> Walker for ResolveNamespace<'a, 'ctxt, 'ra> {
  fn walk_attr<'ast>(&mut self, attr: &'ast Attribute) {}

  fn walk_item<'ast>(&mut self, item: &'ast Item) {
    self.resolve_item(item);
  }
}

#[derive(Debug)]
struct Scope {
  pub bindings: IndexMap<Ident, Res>,
  pub kind: ScopeKind,
}

#[derive(Debug)]
enum ScopeKind {
  Block,
  Korpe,
}
