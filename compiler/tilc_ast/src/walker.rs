use std::fmt::Debug;

use tilc_span::{Ident, Span};

use crate::{
  Attribute, Block, Expression, ExpressionKind, Fn, Item, Local, LocalKind,
  NodeIdx, Path, PathSegment, Pattern, PatternKind, Sandyq, Statement,
  StatementKind, Visibility, VisibilityKind,
};


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

pub trait WalkItemKind: Debug {
  fn mut_walk<W: Walker>(
    &mut self,
    span: Span,
    idx: NodeIdx,
    vis: &mut Visibility,
    walker: &mut W,
  );
}
pub trait Walker: Sized {
  fn mut_walk_attr(&mut self, attr: &mut Attribute) {
    mut_walk_attr(self, attr);
  }

  fn mut_walk_sandyq(&mut self, sandyq: &mut Sandyq) {
    mut_walk_sandyq(self, sandyq);
  }
  fn mut_walk_item<K: WalkItemKind>(&mut self, item: &mut Item<K>) {
    mut_walk_item(self, item);
  }
  fn mut_walk_stmt(&mut self, stmt: &mut Statement) {
    mut_walk_stmt(self, stmt);
  }
  fn mut_walk_expr(&mut self, expr: &mut Expression) {
    mut_walk_expr(self, expr);
  }
  fn mut_walk_pat(&mut self, pat: &mut Pattern) {
    mut_walk_pat(self, pat);
  }
  fn mut_walk_local(&mut self, local: &mut Local) {
    mut_walk_local(self, local);
  }

  fn mut_walk_fn(&mut self, fn_kind: FnKind) {
    mut_walk_fn(self, fn_kind);
  }
  fn mut_walk_block(&mut self, block: &mut Block) {
    mut_walk_block(self, block);
  }

  fn mut_walk_vis(&mut self, vis: &mut Visibility) {
    mut_walk_vis(self, vis);
  }
  fn mut_walk_ident(&mut self, ident: &mut Ident) {
    mut_walk_ident(self, ident);
  }
  fn mut_walk_path(&mut self, path: &mut Path) {
    mut_walk_path(self, path);
  }
  fn mut_walk_path_segment(&mut self, path_segment: &mut PathSegment) {
    mut_walk_path_segment(self, path_segment);
  }

  fn mut_walk_idx(&mut self, idx: &mut NodeIdx) {
    dbg!("Walker's mut_walk_idx did nothing: {}", idx);
  }
  fn mut_walk_span(&mut self, span: &mut Span) {
    dbg!("Walker's mut_walk_span did nothing: {}", span);
  }
}


pub fn mut_walk_attrs<W: Walker>(walker: &mut W, attrs: &mut Vec<Attribute>) {
  dbg!(&attrs);
  for attr in attrs {
    walker.mut_walk_attr(attr);
  }
}
pub fn mut_walk_attr<W: Walker>(walker: &mut W, attr: &mut Attribute) {
  dbg!(&attr);
  let Attribute {
    idx: _,
    path,
    args,
    style: _,
    span,
  } = attr;
  walker.mut_walk_path(path);

  walker.mut_walk_span(span);
}
pub fn mut_walk_sandyq<W: Walker>(walker: &mut W, sandyq: &mut Sandyq) {
  dbg!(&sandyq);
  let Sandyq {
    idx,
    attrs,
    items,
    span,
  } = sandyq;
  walker.mut_walk_idx(idx);
  mut_walk_attrs(walker, attrs);
  for item in items {
    walker.mut_walk_item(item);
  }
  walker.mut_walk_span(span);
}
pub fn mut_walk_item<W: Walker, K: WalkItemKind>(
  walker: &mut W,
  item: &mut Item<K>,
) {
  dbg!(&item);
  let Item {
    idx,
    attrs,
    vis,
    kind,
    ident,
    span,
  } = item;
  walker.mut_walk_idx(idx);
  mut_walk_attrs(walker, attrs);
  kind.mut_walk(*span, *idx, vis, walker);
  walker.mut_walk_ident(ident);
  walker.mut_walk_span(span);
}
pub fn mut_walk_stmt<W: Walker>(walker: &mut W, stmt: &mut Statement) {
  dbg!(&stmt);
  let Statement { idx, kind, span } = stmt;
  walker.mut_walk_idx(idx);
  match kind {
    StatementKind::Item(item) => walker.mut_walk_item(item),
    StatementKind::Expression(expr) | StatementKind::Semi(expr) => {
      walker.mut_walk_expr(expr)
    }
    StatementKind::Let(local) => walker.mut_walk_local(local),
  };
  walker.mut_walk_span(span);
}
pub fn mut_walk_expr<W: Walker>(walker: &mut W, expr: &mut Expression) {
  dbg!(&expr);
  let Expression {
    idx,
    attrs,
    kind,
    span,
  } = expr;
  walker.mut_walk_idx(idx);
  mut_walk_attrs(walker, attrs);

  match kind {
    ExpressionKind::Assign(lhs, span, rhs) => {
      walker.mut_walk_expr(lhs);
      walker.mut_walk_expr(rhs);
      walker.mut_walk_span(span);
    }
    ExpressionKind::Let(pat, expr, span) => {
      walker.mut_walk_pat(pat);
      walker.mut_walk_expr(expr);
      walker.mut_walk_span(span);
    }
    ExpressionKind::Lit(_) => {}
  };
  walker.mut_walk_span(span);
}
pub fn mut_walk_pat<W: Walker>(walker: &mut W, pat: &mut Pattern) {
  dbg!(&pat);
  let Pattern { idx, kind, span } = pat;
  walker.mut_walk_idx(idx);
  match kind {
    PatternKind::Ident(_, ident, pat) => {
      walker.mut_walk_ident(ident);
      if let Some(pat) = pat {
        walker.mut_walk_pat(pat);
      };
    }
    PatternKind::Expression(expr) => {
      walker.mut_walk_expr(expr);
    }

    _ => todo!(),
  };
  walker.mut_walk_span(span);
}
pub fn mut_walk_local<W: Walker>(walker: &mut W, local: &mut Local) {
  dbg!(&local);
  let Local {
    idx,
    attrs,
    pat,
    kind,
    ty,
    span,
  } = local;
  walker.mut_walk_idx(idx);
  mut_walk_attrs(walker, attrs);
  walker.mut_walk_pat(pat);

  match kind {
    LocalKind::Decl => {}
    LocalKind::Init(expr) => {
      walker.mut_walk_expr(expr);
    }
  };

  if let Some(ty) = ty {
    todo!();
  };
  walker.mut_walk_span(span);
}
pub fn mut_walk_fn<W: Walker>(walker: &mut W, fn_kind: FnKind) {
  dbg!(&fn_kind);
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
      if let Some(block) = block {
        walker.mut_walk_block(block);
      };
    }

    FnKind::Closure => todo!(),
  };
}
pub fn mut_walk_block<W: Walker>(walker: &mut W, block: &mut Block) {
  dbg!(&block);
  let Block {
    idx,
    statements,
    span,
  } = block;
  walker.mut_walk_idx(idx);
  for stmt in statements {
    walker.mut_walk_stmt(stmt);
  }
  walker.mut_walk_span(span);
}

pub fn mut_walk_vis<W: Walker>(walker: &mut W, vis: &mut Visibility) {
  dbg!(&vis);
  let Visibility { kind, span } = vis;
  match kind {
    VisibilityKind::Public | VisibilityKind::Private => {}
    VisibilityKind::Protected(idx, path) => {
      walker.mut_walk_idx(idx);
      walker.mut_walk_path(path);
    }
  };
  walker.mut_walk_span(span);
}
pub fn mut_walk_ident<W: Walker>(walker: &mut W, ident: &mut Ident) {
  dbg!(&ident);
  let Ident { name: _, span } = ident;
  walker.mut_walk_span(span);
}
pub fn mut_walk_path<W: Walker>(walker: &mut W, path: &mut Path) {
  dbg!(&path);
  let Path { segments, span } = path;
  for segment in segments {
    walker.mut_walk_path_segment(segment);
  }
  walker.mut_walk_span(span);
}
pub fn mut_walk_path_segment<W: Walker>(
  walker: &mut W,
  path_segment: &mut PathSegment,
) {
  dbg!(&path_segment);
  let PathSegment { ident, idx } = path_segment;
  walker.mut_walk_idx(idx);
  walker.mut_walk_ident(ident);
}
