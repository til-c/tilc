use tilc_ast::{
  Attribute, AttributeStyle, Expr, Local, LocalKind, NodeIdx, Statement, StatementKind, TokenKind,
};
use tilc_error::PResult;
use tilc_span::{Span, kw};

use crate::Parser;

impl<'a> Parser<'a> {
  pub(crate) fn parse_statement(&mut self) -> PResult<'a, Option<Statement>> {
    let mut outer_attrs = self.parse_attributes(AttributeStyle::Outer)?;
    let lo = self.token.span;

    let mut stmt = if self.token.is_kw(kw::Let) {
      self.expect_kw(kw::Let)?;
      let local = self.parse_local(outer_attrs.clone())?;

      Statement {
        idx: NodeIdx::DUMMY,
        kind: StatementKind::Let(local),
        span: lo.to(self.prev_token.span),
      }
    } else if let Some(item) = self.parse_item()? {
      self.make_stmt(
        StatementKind::Item(Box::new(item)),
        lo.to(self.prev_token.span),
      )
    } else if self.eat(TokenKind::Semicolon) {
      return Ok(None);
    } else {
      let expr = self.parse_expr()?;
      let with_semi = self.eat(TokenKind::Semicolon);
      let span = lo.to(expr.span);
      self.make_stmt(
        if with_semi {
          StatementKind::Semi(expr)
        } else {
          StatementKind::Expr(expr)
        },
        span,
      )
    };
    stmt.access_attrs(|attrs| {
      attrs.append(&mut outer_attrs);
    });

    return Ok(Some(stmt));
  }
  fn parse_initializer(&mut self) -> PResult<'a, Option<Box<Expr>>> {
    let is_eq = self.eat(TokenKind::Eq);

    let is_eq_like = matches!(
      self.token.kind,
      TokenKind::EqEq | TokenKind::NotEq | TokenKind::Le | TokenKind::Ge | TokenKind::BinOpEq(..)
    );
    if !is_eq && is_eq_like {
      todo!("cannot use eq like operator on local init");
    };

    return Ok(if is_eq || is_eq_like {
      Some(self.parse_expr()?)
    } else {
      None
    });
  }

  fn parse_local(&mut self, attrs: Vec<Attribute>) -> PResult<'a, Box<Local>> {
    let lo = self.token.span;

    let pat = self.parse_pattern()?;

    let init = match self.parse_initializer() {
      Ok(Some(expr)) => Some(expr),
      Ok(None) => None,

      Err(err) => return Err(err),
    };
    let kind = match init {
      Some(expr) => LocalKind::Init(expr),
      _ => LocalKind::Decl,
    };

    let hi = if self.check(TokenKind::Semicolon) {
      self.token.span
    } else {
      self.prev_token.span
    };
    return Ok(Box::new(Local {
      idx: NodeIdx::DUMMY,

      attrs,
      pat,
      kind,
      ty: None,

      span: lo.to(hi),
    }));
  }
  fn make_stmt(&self, kind: StatementKind, span: Span) -> Statement {
    return Statement {
      idx: NodeIdx::DUMMY,
      kind,
      span,
    };
  }
}
