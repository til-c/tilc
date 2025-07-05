use tilc_ast::{
  Delim, Fn, FnSig, Item, ItemKind, NodeIdx, Sandyq, Statement, TokenKind, Ty,
  TyKind, Visibility, VisibilityKind,
};
use tilc_error::PResult;
use tilc_span::{Ident, Span, Symbol, kw};

use crate::{ItemInfo, Parser};


impl<'a> Parser<'a> {
  pub fn parse_sandyq(&mut self) -> PResult<'a, Sandyq> {
    let (items, span) = self.parse_until(TokenKind::Eof)?;

    return Ok(Sandyq {
      idx: NodeIdx::EMPTY,

      items,

      span,
    });
  }
  pub(crate) fn parse_item(&mut self) -> PResult<'a, Option<Item>> {
    let lo = self.token.span;
    let vis = self.parse_visibility()?;

    let Some((ident, kind)) = self.parse_item_info(lo)? else {
      return Ok(None);
    };

    return Ok(Some(Item {
      idx: NodeIdx::EMPTY,
      ident,
      span: lo.to(self.prev_token.span),
      kind,
      vis,
    }));
  }
  fn parse_item_info(
    &mut self,
    start_span: Span,
  ) -> PResult<'a, Option<ItemInfo>> {
    if self.check(TokenKind::Eof) {
      return Ok(None);
    };

    let item_info = if self.eat_kw(kw::Use) {
      self.parse_use_item()?
    } else if self.check_for_fn_item() {
      self.parse_fn_item(start_span)?
    } else {
      return Ok(None);
    };

    return Ok(Some(item_info));
  }
  fn parse_fn_item(&mut self, start_span: Span) -> PResult<'a, ItemInfo> {
    let fn_header = self.parse_fn_header()?;
    let fn_identifier = self.parse_ident()?;
    let fn_generics = self.parse_fn_generics()?;
    let fn_decl = self.parse_fn_decl()?;

    let sig_hi = self.prev_token.span;
    let fn_body = self.parse_fn_body()?;

    return Ok((
      fn_identifier,
      ItemKind::Fn(Box::new(Fn {
        fn_sig: FnSig {
          fn_header,
          fn_decl,
          span: start_span.to(sig_hi),
        },
        generics: fn_generics,
        block: fn_body,
      })),
    ));
  }
  fn parse_use_item(&mut self) -> PResult<'a, ItemInfo> {
    debug_assert!(self.prev_token.is_kw(kw::Use));
    let use_path = self.parse_use_path()?;
    self.expect(TokenKind::Semicolon)?;


    return Ok((Ident::DUMMY, ItemKind::Use(Box::new(use_path))));
  }
  fn check_for_fn_item(&self) -> bool {
    // TODO: Update the list after adding keywords (if necessary)
    const POSSIBILITIES: &[Symbol; 3] = &[kw::Const, kw::Extern, kw::Async];

    return self.check_kw(kw::Function)
      || (POSSIBILITIES.iter().any(|s| self.check_kw(*s))
        && self.look_ahead(1).is_kw(kw::Function));
  }

  fn parse_visibility(&mut self) -> PResult<'a, Visibility> {
    if !self.eat_kw(kw::Pub) {
      return Ok(Visibility {
        kind: VisibilityKind::Private,
        span: self.prev_token.span.shrink_to_hi(),
      });
    };

    if self.step_if(TokenKind::OpenDelim(Delim::Paren)) {
      todo!();
    };

    return Ok(Visibility {
      kind: VisibilityKind::Public,
      span: self.prev_token.span,
    });
  }
  pub(crate) fn parse_ty(&mut self) -> PResult<'a, Box<Ty>> {
    let lo = self.token.span;

    let ty_kind = if self.step_if(TokenKind::Not) {
      TyKind::Never
    } else if self.eat_kw(kw::Underscore) {
      TyKind::Infer
    } else if let Some((..)) = self.token.ident() {
      todo!()
      // let path: Box<Path> = Box::new(self.parse_path()?);
      // TyKind::Path(path)
    } else {
      todo!()
    };

    return Ok(Box::new(Ty {
      idx: NodeIdx::EMPTY,
      kind: ty_kind,
      span: lo.to(self.prev_token.span),
    }));
  }
  pub(crate) fn parse_statement(&mut self) -> PResult<'a, Option<Statement>> {
    dbg!("{:#?}", &self.token_cursor);

    Ok(None)
  }
}
