use crate::Parser;

use tilc_ast::{
  Block, Delim, Fn, FnDecl, FnHeader, FnReturnType, FnSig, Generics, Item,
  ItemInfo, ItemKind, NodeIdx, Param, Sandyq, Statement, TokenKind, Ty, TyKind,
  Visibility, VisibilityKind,
};
use tilc_errors::PResult;
use tilc_span::{kw, Identifier, Span, Symbol};


impl<'a> Parser<'a> {
  pub fn parse_sandyq(&mut self) -> PResult<'a, Sandyq> {
    let items: Vec<Item> = self.parse_until(&TokenKind::Eof)?;

    return Ok(Sandyq {
      idx: NodeIdx::EMPTY,

      attributes: Vec::new(),

      items,
    });
  }
  pub(crate) fn parse_until(
    &mut self,
    stopper: &TokenKind,
  ) -> PResult<'a, Vec<Item>> {
    let mut items: Vec<Item> = Vec::new();

    loop {
      let Some(item): Option<Item> = self.parse_item()? else {
        break;
      };
      items.push(item);
    }

    if !self.step_if(stopper) {
      todo!();
    };


    return Ok(items);
  }

  pub(crate) fn check_for_fn_item(&self) -> bool {
    // TODO: Update the list after adding keywords (if necessary)
    const POSSIBILITIES: &[Symbol] = &[kw::Const, kw::Extern, kw::Async];

    return self.check_kw(kw::Function)
      || (POSSIBILITIES.iter().any(|s: &Symbol| self.check_kw(*s))
        && self.look_ahead(1).is_kw(kw::Function));
  }

  pub(crate) fn parse_visibility(&mut self) -> PResult<'a, Visibility> {
    if !self.eat_kw(kw::Pub) {
      return Ok(Visibility {
        kind: VisibilityKind::Private,
        span: self.prev_token.span.shrink_to_hi(),
      });
    };

    if self.step_if(&TokenKind::OpenDelim(Delim::Paren)) {
      todo!();
    };

    return Ok(Visibility {
      kind: VisibilityKind::Public,
      span: self.prev_token.span,
    });
  }
  pub(crate) fn parse_item_info(
    &mut self,
    start_span: Span,
  ) -> PResult<'a, Option<ItemInfo>> {
    if self.check(&TokenKind::Eof) {
      return Ok(None);
    };

    let item_info: ItemInfo = if self.check_kw(kw::Use) {
      self.parse_use_item()?
    } else if self.check_for_fn_item() {
      self.parse_fn_item(start_span)?
    } else {
      return Ok(None);
    };

    return Ok(Some(item_info));
  }
  pub(crate) fn parse_item(&mut self) -> PResult<'a, Option<Item>> {
    let start_span: Span = self.token.span;
    let vis: Visibility = self.parse_visibility()?;

    let Some((ident, kind)): Option<ItemInfo> =
      self.parse_item_info(start_span)?
    else {
      return Ok(None);
    };

    return Ok(Some(Item {
      idx: NodeIdx::EMPTY,
      ident,
      span: start_span.to(self.token.span),
      kind,
      visibility: vis,
    }));
  }


  fn parse_ident(&mut self) -> PResult<'a, Identifier> {
    let (ident, raw): (Identifier, bool) = match self.token.ident() {
      Some((ident, raw)) => (ident, raw),
      _ => todo!(),
    };
    if !raw && ident.is_reserved() {
      todo!();
    };

    self.step();
    return Ok(ident);
  }

  pub(crate) fn parse_use_item(&mut self) -> PResult<'a, ItemInfo> {
    todo!()
  }
  pub(crate) fn parse_fn_item(
    &mut self,
    start_span: Span,
  ) -> PResult<'a, ItemInfo> {
    let fn_header: FnHeader = self.parse_fn_header()?;
    let fn_identifier: Identifier = self.parse_ident()?;
    let fn_generics: Generics = self.parse_fn_generics()?;
    let fn_decl: FnDecl = self.parse_fn_decl()?;

    let sig_hi: Span = self.prev_token.span;
    let fn_body: Option<Block> = self.parse_fn_body()?;

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
  pub(crate) fn parse_fn_header(&mut self) -> PResult<'a, FnHeader> {
    // (turaqty | `async`) (qauipti | qauipsiz) fx myrqymbai() {}
    let is_const: bool = self.eat_kw(kw::Const);
    let is_async: bool = self.eat_kw(kw::Async);

    if !self.eat_kw(kw::Function) {
      todo!()
      // return Err(self.dcx().create_error());
    };

    return Ok(FnHeader { is_const, is_async });
  }
  pub(crate) fn parse_fn_decl(&mut self) -> PResult<'a, FnDecl> {
    self.expect(&TokenKind::OpenDelim(Delim::Paren))?;

    let mut params: Vec<Param> = Vec::new();
    if !self.step_if(&TokenKind::CloseDelim(Delim::Paren)) {
      self.parse_inside_delim(Delim::Paren, |this: &mut Self| {
        return Ok(());
      })?;
      todo!()
    };

    let return_ty: FnReturnType = self.parse_fn_return_ty()?;

    return Ok(FnDecl { params, return_ty });
  }
  pub(crate) fn parse_fn_generics(&mut self) -> PResult<'a, Generics> {
    if !self.eat_token_kind(TokenKind::Lt) {
      return Ok(Generics {
        params: Vec::new(),
        span: self.prev_token.span.shrink_to_hi(),
      });
    };

    todo!()
  }
  pub(crate) fn parse_fn_body(&mut self) -> PResult<'a, Option<Block>> {
    if self.check(&TokenKind::Semicolon) {
      return Ok(None);
    };
    let lo_span: Span = self.token.span;
    self.expect(&TokenKind::OpenDelim(Delim::Brace))?;

    let mut statements: Vec<Statement> = Vec::new();
    while !self.eat(&TokenKind::CloseDelim(Delim::Brace)) {
      if self.check(&TokenKind::Eof) {
        break;
      };

      let statement: Option<Statement> = self.parse_statement()?;

      if let Some(statement) = statement {
        statements.push(statement);
      } else {
        // Skip repetive ';' (if any)
        continue;
      };
    }

    return Ok(Some(Block {
      idx: NodeIdx::EMPTY,

      statements,
      span: lo_span.to(self.prev_token.span),
    }));
  }
  pub(crate) fn parse_fn_return_ty(&mut self) -> PResult<'a, FnReturnType> {
    if !self.step_if(&TokenKind::RArrow) {
      return Ok(FnReturnType::Default);
    };
    todo!();
  }


  pub(crate) fn parse_ty(&mut self) -> PResult<'a, Box<Ty>> {
    use TokenKind::*;

    let ty_kind: TyKind = match self.token.kind {
      Not => {
        self.step();
        TyKind::Never
      }
      Identifier(symbol, is_raw) => todo!(),

      _ => todo!(),
    };

    todo!();
  }

  /// Assumes that prev token kind is open paranthesis
  pub(crate) fn parse_inside_delim<T, F: FnMut(&mut Self) -> PResult<'a, T>>(
    &mut self,
    delim: Delim,
    mut f: F,
  ) -> PResult<'a, T> {
    debug_assert_eq!(self.prev_token.kind, TokenKind::OpenDelim(delim));
    let res: PResult<'a, T> = f(self);

    self.expect(&TokenKind::CloseDelim(delim))?;
    return res;
  }
  pub(crate) fn parse_statement(&mut self) -> PResult<'a, Option<Statement>> {
    // todo!()
    Ok(None)
  }
}
