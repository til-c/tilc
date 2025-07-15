use tilc_span::{Ident, Span};

use crate::{Fn, Item, NodeIdx, Sandyq, Visibility};


#[derive(Debug)]
pub enum FnKind<'a> {
  Fn(FnCtxt, &'a mut Visibility, &'a mut Fn),
  Closure,
}
#[derive(Debug)]
pub enum FnCtxt {
  Free,
  Foreign,
  Assoc(AssocCtxt),
}
#[derive(Debug)]
pub enum AssocCtxt {
  Trait,
  Impl(bool),
}

pub trait WalkItemKind {
  fn mut_walk<W: Walker>(
    &mut self,
    span: Span,
    idx: NodeIdx,
    vis: &mut Visibility,
    walker: &mut W,
  );
}
pub trait Walker: Sized {
  fn mut_walk_sandyq(&mut self, sandyq: &mut Sandyq) {
    mut_walk_sandyq(self, sandyq);
  }
  fn mut_walk_item<K: WalkItemKind>(&mut self, item: &mut Item<K>) {
    let Item {
      idx,
      attrs,
      vis,
      kind,
      ident,
      span,
    } = item;
    self.mut_walk_idx(idx);
    kind.mut_walk(*span, *idx, vis, self);
  }
  fn mut_walk_fn(&mut self, fn_kind: FnKind) {
    mut_walk_fn(self, fn_kind);
  }
  fn mut_walk_ident(&mut self, ident: &mut Ident) {}

  fn mut_walk_idx(&mut self, idx: &mut NodeIdx) {
    dbg!("Walker's mut_walk_idx did nothing: {}", idx);
  }
  fn mut_walk_span(&mut self, span: &mut Span) {
    dbg!("Walker's mut_walk_span did nothing: {}", span);
  }
}

pub fn mut_walk_sandyq<W: Walker>(walker: &mut W, sandyq: &mut Sandyq) {
  let Sandyq { idx, items, .. } = sandyq;
  walker.mut_walk_idx(idx);
  _ = items.iter_mut().map(|item| walker.mut_walk_item(item));
}
pub fn mut_walk_fn<W: Walker>(walker: &mut W, fn_kind: FnKind) {
  match fn_kind {
    FnKind::Fn(
      _,
      _,
      Fn {
        generics,
        ident,
        fn_sig,
        block,
      },
    ) => {
      walker.mut_walk_ident(ident);
    }

    FnKind::Closure => todo!(),
  };
}
pub fn mut_walk_ident<W: Walker>(
  walker: &mut W,
  Ident { name: _, span }: &mut Ident,
) {
  walker.mut_walk_span(span);
}
