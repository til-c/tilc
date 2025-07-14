use tilc_ast::{
  Attribute, AttributeStyle, Expression, ExpressionKind, NodeIdx, TokenKind,
};
use tilc_error::PResult;
use tilc_span::{Span, kw};

use crate::Parser;


impl<'a> Parser<'a> {
  pub(crate) fn parse_expression(&mut self) -> PResult<'a, Box<Expression>> {
    let outer_attrs = self.parse_attributes(AttributeStyle::Outer)?;

    if matches!(self.token.kind, TokenKind::Literal(..)) {
      return self.parse_expr_lit(outer_attrs);
    } else if self.check_kw(kw::Let) {
      return self.parse_expr_let(outer_attrs);
    };

    todo!();
  }

  fn parse_expr_lit(
    &mut self,
    attrs: Vec<Attribute>,
  ) -> PResult<'a, Box<Expression>> {
    let lo = self.token.span;

    let lit = match self.token.kind {
      TokenKind::Literal(lit) => {
        self.step();
        lit
      }
      _ => todo!(),
    };

    return Ok(self.make_expr(
      attrs,
      ExpressionKind::Lit(lit),
      lo.to(self.prev_token.span),
    ));
  }
  fn parse_expr_let(
    &mut self,
    attrs: Vec<Attribute>,
  ) -> PResult<'a, Box<Expression>> {
    let lo = self.token.span;


    self.expect_kw(kw::Let)?;
    let pat = self.parse_pattern()?;

    self.expect(TokenKind::Eq)?;
    let expr = self.parse_expression()?;
    let span = lo.to(expr.span);


    return Ok(Box::new(Expression {
      idx: NodeIdx::DUMMY,
      attrs,
      kind: ExpressionKind::Let(pat, expr, span),
      span,
    }));
  }
  fn make_expr(
    &self,
    attrs: Vec<Attribute>,
    kind: ExpressionKind,
    span: Span,
  ) -> Box<Expression> {
    return Box::new(Expression {
      idx: NodeIdx::DUMMY,
      attrs,
      kind,
      span,
    });
  }
}
