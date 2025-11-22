use tilc_ast::{
  Attribute, AttributeStyle, Delim, Fn, FnSig, Item, ItemKind, NodeIdx, Sandyq, TokenKind, Ty,
  TyKind, Vis, VisKind,
};
use tilc_error::PResult;
use tilc_span::{Ident, Span, Symbol, kw};

use crate::Parser;

pub(crate) type ItemInfo = (Ident, ItemKind);

impl<'a> Parser<'a> {
  pub fn parse_sandyq(&mut self) -> PResult<'a, Sandyq> {
    let (items, attrs, span) = self.parse_until(TokenKind::Eof)?;

    Ok(Sandyq {
      idx: NodeIdx::DUMMY,

      attrs,
      items,

      span,
    })
  }
  fn parse_until(&mut self, stopper: TokenKind) -> PResult<'a, (Vec<Item>, Vec<Attribute>, Span)> {
    let lo = self.token.span;
    let mut items = Vec::new();
    let attrs = self.parse_attributes(AttributeStyle::Inner)?;

    loop {
      let Some(item): Option<Item> = self.parse_item()? else {
        break;
      };
      items.push(item);
    }

    let span = lo.to(self.prev_token.span);
    if !self.eat(stopper) {
      todo!();
    };

    Ok((items, attrs, span))
  }

  pub(crate) fn parse_item(&mut self) -> PResult<'a, Option<Item>> {
    let lo = self.token.span;
    let vis = self.parse_vis()?;

    let Some((ident, kind)) = self.parse_item_info(lo)? else {
      return Ok(None);
    };

    Ok(Some(Item {
      idx: NodeIdx::DUMMY,

      attrs: Vec::new(),
      vis,
      kind,
      ident,

      span: lo.to(self.prev_token.span),
    }))
  }
  fn parse_item_info(&mut self, start_span: Span) -> PResult<'a, Option<ItemInfo>> {
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

    Ok(Some(item_info))
  }

  fn parse_fn_item(&mut self, start_span: Span) -> PResult<'a, ItemInfo> {
    let fn_header = self.parse_fn_header()?;
    let fn_identifier = self.parse_ident()?;
    let fn_generics = self.parse_fn_generics()?;
    let fn_decl = self.parse_fn_decl()?;

    let sig_hi = self.prev_token.span;
    let fn_body = self.parse_fn_body()?;

    Ok((
      fn_identifier.clone(),
      ItemKind::Fn(Box::new(Fn {
        ident: fn_identifier,
        fn_sig: FnSig {
          fn_header,
          fn_decl,
          span: start_span.to(sig_hi),
        },
        generics: fn_generics,
        block: fn_body,
      })),
    ))
  }
  fn parse_use_item(&mut self) -> PResult<'a, ItemInfo> {
    debug_assert!(self.prev_token.is_kw(kw::Use));
    let use_path = self.parse_use_path()?;
    self.expect(TokenKind::Semicolon)?;

    Ok((Ident::DUMMY, ItemKind::Use(Box::new(use_path))))
  }

  pub(crate) fn parse_ident(&mut self) -> PResult<'a, Ident> {
    let (ident, raw) = match self.token.ident() {
      Some((ident, raw)) => (ident, raw),
      None => {
        // TODO: Error handling
        todo!();
      }
    };
    if !raw && ident.is_reserved() {
      todo!();
    };

    self.step();
    Ok(ident)
  }

  fn parse_vis(&mut self) -> PResult<'a, Vis> {
    if !self.eat_kw(kw::Pub) {
      return Ok(Vis {
        kind: VisKind::Private,
        span: self.prev_token.span.shrink_to_hi(),
      });
    };

    if self.eat(TokenKind::OpenDelim(Delim::Paren)) {
      todo!();
    };

    Ok(Vis {
      kind: VisKind::Public,
      span: self.prev_token.span,
    })
  }

  fn check_for_fn_item(&self) -> bool {
    // TODO: Update the list after adding keywords (if necessary)
    // const POSSIBILITIES: &[Symbol; 3] = &[kw::Const, kw::Extern, kw::Async];
    const POSSIBILITIES: &[Symbol; 1] = &[kw::Const];

    self.check_kw(kw::Function)
      || (POSSIBILITIES.iter().any(|s| self.check_kw(*s)) && self.look_ahead(1).is_kw(kw::Function))
  }

  pub(crate) fn parse_ty(&mut self) -> PResult<'a, Box<Ty>> {
    let lo = self.token.span;

    let ty_kind = if self.eat(TokenKind::Bang) {
      TyKind::Never
    } else if self.eat_kw(kw::Underscore) {
      TyKind::Infer
    } else if let Some((..)) = self.token.ident() {
      let path = Box::new(self.parse_path()?);
      TyKind::Path(path)
    } else {
      todo!()
    };

    Ok(Box::new(Ty {
      idx: NodeIdx::DUMMY,
      kind: ty_kind,
      span: lo.to(self.prev_token.span),
    }))
  }
}
