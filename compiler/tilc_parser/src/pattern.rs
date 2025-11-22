use tilc_ast::{Mutability, NodeIdx, Pattern, PatternKind, TokenKind};
use tilc_error::PResult;
use tilc_span::{Span, kw};

use crate::Parser;

impl<'a> Parser<'a> {
  pub(crate) fn parse_pattern(&mut self) -> PResult<'a, Box<Pattern>> {
    let lo = self.token.span;

    let kind = if self.check_kw(kw::Mut) {
      self.expect_kw(kw::Mut)?;
      self.parse_pat_ident(Mutability::Mut)?
    } else if self.token.ident().is_some() {
      self.parse_pat_ident(Mutability::Nope)?
    } else {
      todo!();
    };

    return Ok(self.make_pat(kind, lo.to(self.token.span)));
  }

  fn parse_pat_ident(&mut self, mutability: Mutability) -> PResult<'a, PatternKind> {
    let ident = self.parse_ident()?;

    let pat = if self.eat(TokenKind::At) {
      Some(self.parse_pattern()?)
    } else {
      None
    };
    if pat.is_some() {
      todo!();
    };

    return Ok(PatternKind::Ident(mutability, ident, pat));
  }

  fn make_pat(&self, kind: PatternKind, span: Span) -> Box<Pattern> {
    return Box::new(Pattern {
      idx: NodeIdx::DUMMY,
      kind,
      span,
    });
  }
}
