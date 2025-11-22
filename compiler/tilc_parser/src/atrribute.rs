use tilc_ast::{AttrArgs, Attribute, AttributeStyle, Delim, Path, TokenKind};
use tilc_error::PResult;

use crate::Parser;

impl<'a> Parser<'a> {
  pub(crate) fn parse_attributes(
    &mut self,
    attr_style: AttributeStyle,
  ) -> PResult<'a, Vec<Attribute>> {
    match attr_style {
      AttributeStyle::Inner => return self.parse_inner_attrs(),
      AttributeStyle::Outer => return self.parse_outer_attrs(),
    };
  }
  fn parse_inner_attrs(&mut self) -> PResult<'a, Vec<Attribute>> {
    let mut attrs = Vec::new();

    loop {
      let attr = if self.check(TokenKind::Hashtag)
        && self.look_ahead_and(1, |t| t.kind == TokenKind::Bang)
      {
        Some(self.parse_attr()?)
      } else {
        None
      };

      if let Some(attr) = attr {
        attrs.push(attr);
      } else {
        break;
      };
    }

    return Ok(attrs);
  }
  fn parse_outer_attrs(&mut self) -> PResult<'a, Vec<Attribute>> {
    let mut attrs = Vec::new();

    loop {
      let attr = if self.check(TokenKind::Hashtag) {
        Some(self.parse_attr()?)
      } else {
        None
      };

      if let Some(attr) = attr {
        if matches!(attr.style, AttributeStyle::Outer) {
          attrs.push(attr);
        };
      } else {
        break;
      };
    }

    return Ok(attrs);
  }

  fn parse_attr(&mut self) -> PResult<'a, Attribute> {
    let lo = self.token.span;

    self.expect(TokenKind::Hashtag)?;
    let attr_style = if self.eat(TokenKind::Bang) {
      AttributeStyle::Inner
    } else {
      AttributeStyle::Outer
    };
    self.expect(TokenKind::OpenDelim(Delim::Bracket))?;
    let (path, args) = self.parse_attr_item()?;
    self.expect(TokenKind::CloseDelim(Delim::Bracket))?;
    let attr_span = lo.to(self.prev_token.span);

    return Ok(Attribute {
      idx: self.psess.make_attr_idx(),

      path,
      args,

      style: attr_style,
      span: attr_span,
    });
  }
  fn parse_attr_item(&mut self) -> PResult<'a, (Path, AttrArgs)> {
    let path = self.parse_path()?;
    let args = self.parse_attr_args()?;

    return Ok((path, args));
  }
  fn parse_attr_args(&mut self) -> PResult<'a, AttrArgs> {
    if matches!(self.token.kind, TokenKind::OpenDelim(_)) {
      todo!();
    } else if self.eat(TokenKind::Eq) {
      let eq_span = self.prev_token.span;
      return Ok(AttrArgs::Eq {
        eq_span,
        expr: self.parse_expr()?,
      });
    } else {
      return Ok(AttrArgs::Empty);
    };
  }
}
