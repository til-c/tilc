use crate::Parser;

use tilc_ast::{
  Block, Delim, Fn, FnDecl, FnHeader, FnReturnType, FnSig, Generics, Item,
  ItemInfo, ItemKind, NodeIdx, Param, Sandyq, Statement, Token, TokenKind,
  Visibility, VisibilityKind,
};
use tilc_errors::PResult;
use tilc_span::{kw, Identifier, Span, Symbol};


impl<'a> Parser<'a> {
  pub fn expect(&mut self, token_kind: &TokenKind) -> PResult<'a, Token> {
    if self.check(token_kind) {
      self.step();
      return Ok(self.token);
    } else {
      todo!();
    };
  }
  pub fn expect_any_of(
    &mut self,
    expectations: &[TokenKind],
  ) -> PResult<'a, Token> {
    if expectations.contains(&self.token.kind) {
      self.step();
      return Ok(self.prev_token);
    } else {
      todo!()
    };
  }
  pub fn eat_token_kind(&mut self, token_kind: TokenKind) -> bool {
    if self.token.kind == token_kind {
      self.step();
      return true;
    };
    return false;
  }

  pub fn check_for_fn_item(&self) -> bool {
    // TODO: Update the list after adding keywords (if necessary)
    const POSSIBILITIES: &[Symbol] = &[kw::Const, kw::Extern];

    return self.check_kw(kw::Function)
      || (POSSIBILITIES.iter().any(|s: &Symbol| self.check_kw(*s))
        && self.look_ahead(1).is_kw(kw::Function))
      || todo!();
  }


  pub fn parse_sandyq(&mut self) -> PResult<'a, Sandyq> {
    let items: Vec<Item> = self.parse_until(&TokenKind::Eof)?;

    return Ok(Sandyq {
      idx: NodeIdx::EMPTY,

      attributes: Vec::new(),

      items,
    });
  }
  pub fn parse_until(&mut self, stopper: &TokenKind) -> PResult<'a, Vec<Item>> {
    let mut items: Vec<Item> = Vec::new();

    loop {
      let Some(item): Option<Item> = self.parse_item()? else {
        break;
      };
      items.push(item);
    }

    if !self.step_if(stopper) {
      return Err(todo!());
    };


    return Ok(items);
  }
  pub fn parse_visibility(&mut self) -> PResult<'a, Visibility> {
    if !self.eat_kw(kw::Pub) {
      return Ok(Visibility {
        kind: VisibilityKind::Private,
        span: self.prev_token.span.shrink_to_hi(),
      });
    };

    todo!()
  }
  pub fn parse_item_info(
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
  pub fn parse_item(&mut self) -> PResult<'a, Option<Item>> {
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


    return Ok(ident);
  }

  pub fn parse_use_item(&mut self) -> PResult<'a, ItemInfo> {
    todo!()
  }
  pub fn parse_fn_item(&mut self, start_span: Span) -> PResult<'a, ItemInfo> {
    let fn_header: FnHeader = self.parse_fn_header()?;
    let fn_identifier: Identifier = self.parse_ident()?;
    let fn_generics: Generics = self.parse_fn_generics()?;
    let fn_decl: FnDecl = self.parse_fn_decl()?;

    let sig_hi: Span = self.prev_token.span;
    let fn_body: Option<Block> = self.parse_fn_body()?;

    return Ok((
      fn_identifier,
      ItemKind::Fn(Fn {
        fn_sig: FnSig {
          fn_header,
          fn_decl,
          span: start_span.to(sig_hi),
        },
        generics: fn_generics,
        block: fn_body,
      }),
    ));
  }
  fn parse_fn_header(&mut self) -> PResult<'a, FnHeader> {
    // (turaqty | `async`) (qauipti | qauipsiz) fx myrqymbai() {}
    let is_const: bool = self.token.is_kw(kw::Const);
    if !self.eat_kw(kw::Function) {
      todo!()
    };

    return Ok(FnHeader { is_const });
  }
  fn parse_fn_decl(&mut self) -> PResult<'a, FnDecl> {
    self.expect(&TokenKind::OpenDelim(Delim::Paren))?;

    let mut params: Vec<Param> = Vec::new();
    if !self.step_if(&TokenKind::CloseDelim(Delim::Paren)) {
      todo!()
    };

    let return_ty: FnReturnType = self.parse_fn_return_ty()?;

    return Ok(FnDecl { params, return_ty });
  }
  fn parse_fn_generics(&mut self) -> PResult<'a, Generics> {
    self.step();
    if !self.eat_token_kind(TokenKind::Lt) {
      return Ok(Generics {
        params: Vec::new(),
        span: self.prev_token.span.shrink_to_hi(),
      });
    };

    todo!()
  }
  fn parse_fn_body(&mut self) -> PResult<'a, Option<Block>> {
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
  fn parse_fn_return_ty(&mut self) -> PResult<'a, FnReturnType> {
    if !self.check(&TokenKind::RArrow) {
      return Ok(FnReturnType::Default);
    };
    todo!();
  }


  fn parse_statement(&mut self) -> PResult<'a, Option<Statement>> {
    // todo!()
    Ok(None)
  }
}
