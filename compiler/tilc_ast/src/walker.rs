use std::fmt::Debug;

use tilc_span::{Ident, Span};

use crate::{
  AttrArgs, Attribute, Block, Expression, ExpressionKind, Fn, Item, Local,
  LocalKind, NodeIdx, Path, PathSegment, Pattern, PatternKind, Sandyq,
  Statement, StatementKind, Visibility, VisibilityKind,
};


#[derive(Debug)]
pub enum FnKindMut<'a> {
  Fn(FnCtxt, &'a mut Visibility, &'a mut Fn),
  Closure,
}
#[derive(Debug)]
pub enum FnKind<'a> {
  Fn(FnCtxt, &'a Visibility, &'a Fn),
  Closure,
}
#[derive(Debug)]
pub enum FnCtxt {
  // plain fx
  Free,

  // fx from extern item
  Foreign,

  // fx from trait or impl item
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
  fn walk<W: Walker>(
    &self,
    span: Span,
    idx: NodeIdx,
    vis: &Visibility,
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
  fn mut_walk_item(&mut self, item: &mut Item) {
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

  fn mut_walk_fn(&mut self, fn_kind: FnKindMut) {
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

  fn walk_attr<'ast>(&mut self, attr: &'ast Attribute) {
    walk_attr(self, attr);
  }

  fn walk_sandyq<'ast>(&mut self, sandyq: &'ast Sandyq) {
    walk_sandyq(self, sandyq);
  }
  fn walk_item<'ast>(&mut self, item: &'ast Item) {
    walk_item(self, item);
  }
  fn walk_stmt<'ast>(&mut self, stmt: &'ast Statement) {
    walk_stmt(self, stmt);
  }
  fn walk_expr<'ast>(&mut self, expr: &'ast Expression) {
    walk_expr(self, expr);
  }
  fn walk_pat<'ast>(&mut self, pat: &'ast Pattern) {
    walk_pat(self, pat);
  }
  fn walk_local<'ast>(&mut self, local: &'ast Local) {
    walk_local(self, local);
  }

  fn walk_fn<'ast>(&mut self, fn_kind: &'ast FnKind) {
    walk_fn(self, fn_kind);
  }
  fn walk_block<'ast>(&mut self, block: &'ast Block) {
    walk_block(self, block);
  }

  fn walk_vis<'ast>(&mut self, vis: &'ast Visibility) {}
  fn walk_ident<'ast>(&mut self, ident: &'ast Ident) {}
  fn walk_path<'ast>(&mut self, path: &'ast Path) {}
  fn walk_path_segment<'ast>(&mut self, path_segment: &'ast PathSegment) {}

  fn walk_idx<'ast>(&mut self, idx: &'ast NodeIdx) {}
  fn walk_span<'ast>(&mut self, span: &'ast Span) {}
}


pub fn mut_walk_attrs<W: Walker>(walker: &mut W, attrs: &mut Vec<Attribute>) {
  dbg!("Mut walker: ", &attrs);
  for attr in attrs {
    walker.mut_walk_attr(attr);
  }
}
pub fn mut_walk_attr<W: Walker>(walker: &mut W, attr: &mut Attribute) {
  dbg!("Mut walker: ", &attr);
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
  dbg!("Mut walker: ", &sandyq);
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
  dbg!("Mut walker: ", &item);
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
  dbg!("Mut walker: ", &stmt);
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
  dbg!("Mut walker: ", &expr);
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
  dbg!("Mut walker: ", &pat);
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
  dbg!("Mut walker: ", &local);
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
    LocalKind::InitElse(expr, block) => {
      walker.mut_walk_expr(expr);
      walker.mut_walk_block(block);
    }
  };

  if let Some(ty) = ty {
    todo!();
  };
  walker.mut_walk_span(span);
}
pub fn mut_walk_fn<W: Walker>(walker: &mut W, fn_kind: FnKindMut<'_>) {
  dbg!("Mut walker: ", &fn_kind);
  match fn_kind {
    FnKindMut::Fn(
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

    FnKindMut::Closure => todo!(),
  };
}
pub fn mut_walk_block<W: Walker>(walker: &mut W, block: &mut Block) {
  dbg!("Mut walker: ", &block);
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
  dbg!("Mut walker: ", &vis);
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
  dbg!("Mut walker: ", &ident);
  let Ident { name: _, span } = ident;
  walker.mut_walk_span(span);
}
pub fn mut_walk_path<W: Walker>(walker: &mut W, path: &mut Path) {
  dbg!("Mut walker: ", &path);
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
  dbg!("Mut walker: ", &path_segment);
  let PathSegment { ident, idx } = path_segment;
  walker.mut_walk_idx(idx);
  walker.mut_walk_ident(ident);
}


pub fn walk_attrs<'ast, W: Walker>(
  walker: &mut W,
  attrs: &'ast Vec<Attribute>,
) {
  dbg!("Walker: ", &attrs);
  for attr in attrs {
    walker.walk_attr(attr);
  }
}
pub fn walk_attr<'ast, W: Walker>(walker: &mut W, attr: &'ast Attribute) {
  dbg!("Walker: ", &attr);
  let Attribute {
    idx: _,
    path,
    args,
    style: _,
    span,
  } = attr;
  walker.walk_path(path);
  match args {
    AttrArgs::Empty | AttrArgs::Delimited(_) => {}
    AttrArgs::Eq { eq_span, expr } => {
      walker.walk_expr(expr);
      walker.walk_span(eq_span);
    }
  }
  walker.walk_span(span);
}

pub fn walk_sandyq<'ast, W: Walker>(walker: &mut W, sandyq: &'ast Sandyq) {
  dbg!("Walker: ", &sandyq);
  let Sandyq {
    idx: _,
    attrs,
    items,
    span: _,
  } = sandyq;
  walk_attrs(walker, attrs);
  for item in items {
    walker.walk_item(item);
  }
}
pub fn walk_item<'ast, W: Walker, K: WalkItemKind>(
  walker: &mut W,
  item: &Item<K>,
) {
  dbg!("Walker: ", &item);
  let Item {
    idx,
    attrs,
    vis,
    kind,
    ident: _,
    span,
  } = item;
  walker.walk_idx(idx);
  walk_attrs(walker, attrs);
  walker.walk_vis(vis);
  kind.walk(*span, *idx, vis, walker);
  walker.walk_span(span);
}
pub fn walk_stmt<'ast, W: Walker>(walker: &mut W, stmt: &'ast Statement) {
  dbg!("Walker: ", &stmt);
  let Statement { idx, kind, span } = stmt;
  walker.walk_idx(idx);
  match kind {
    StatementKind::Let(local) => walker.walk_local(local),
    StatementKind::Item(item) => walker.walk_item(item),
    StatementKind::Expression(expr) | StatementKind::Semi(expr) => {
      walker.walk_expr(expr)
    }
  };
  walker.walk_span(span);
}
pub fn walk_expr<'ast, W: Walker>(walker: &mut W, expr: &'ast Expression) {
  dbg!("Walker: ", &expr);
  let Expression {
    idx,
    attrs,
    kind,
    span,
  } = expr;
  walker.walk_idx(idx);
  walk_attrs(walker, attrs);
  match kind {
    ExpressionKind::Let(pat, expr, span) => {
      walker.walk_pat(pat);
      walker.walk_expr(expr);
      walker.walk_span(span);
    }
    ExpressionKind::Assign(lhs, eq_span, rhs) => {
      walker.walk_expr(lhs);
      walker.walk_expr(rhs);
      walker.walk_span(eq_span);
    }
    ExpressionKind::Lit(_) => {}
  }
  walker.walk_span(span);
}
pub fn walk_pat<'ast, W: Walker>(walker: &mut W, pat: &'ast Pattern) {
  dbg!("Walker: ", &pat);
  let Pattern { idx, kind, span } = pat;
  walker.walk_idx(idx);
  match kind {
    PatternKind::Ident(_, ident, sub_pat) => {
      walker.walk_ident(ident);
      if let Some(sub_pat) = sub_pat {
        walker.walk_pat(sub_pat);
      };
    }
    PatternKind::Expression(expr) => {
      walker.walk_expr(expr);
    }
    _ => todo!(),
  };
  walker.walk_span(span);
}
pub fn walk_local<'ast, W: Walker>(walker: &mut W, local: &'ast Local) {
  dbg!("Walker: ", &local);
  let Local {
    idx,
    attrs,
    pat,
    kind,
    ty,
    span,
  } = local;
  walker.walk_idx(idx);
  walk_attrs(walker, attrs);
  walker.walk_pat(pat);
  if let Some(ty) = ty {
    todo!();
  };

  match kind {
    LocalKind::Decl => {}
    LocalKind::Init(expr) => {
      walker.walk_expr(expr);
    }
    LocalKind::InitElse(expr, block) => {
      walker.walk_expr(expr);
      walker.walk_block(block);
    }
  };
  walker.walk_span(span);
}

pub fn walk_fn<'ast, W: Walker>(walker: &mut W, fn_kind: &'ast FnKind) {
  dbg!("Walker: ", &fn_kind);
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
      walker.walk_ident(ident);
      if let Some(block) = block {
        walker.walk_block(block);
      };
    }

    FnKind::Closure => todo!(),
  };
}
pub fn walk_block<'ast, W: Walker>(walker: &mut W, block: &'ast Block) {
  dbg!("Walker: ", &block);
  let Block {
    idx,
    statements,
    span,
  } = block;
  walker.walk_idx(idx);
  for stmt in statements {
    walker.walk_stmt(stmt);
  }
  walker.walk_span(span);
}
