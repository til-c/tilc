use tilc_ast::{
  Block, Delim, FnDecl, FnHeader, FnReturnType, Generics, NodeIdx, TokenKind, TyKind,
};
use tilc_error::PResult;
use tilc_span::kw;

use crate::Parser;

impl<'a> Parser<'a> {
  pub(crate) fn parse_fn_header(&mut self) -> PResult<'a, FnHeader> {
    // (turaqty | `async`) (qauipti | qauipsiz) fx myrqymbai() {}
    let is_const = self.eat_kw(kw::Const);
    // let is_async = self.eat_kw(kw::Async);
    let is_async = false;

    if !self.eat_kw(kw::Function) {
      todo!()
      // return Err(self.dcx().create_error());
    };

    return Ok(FnHeader { is_const, is_async });
  }
  pub(crate) fn parse_fn_decl(&mut self) -> PResult<'a, FnDecl> {
    self.expect(TokenKind::OpenDelim(Delim::Paren))?;

    let mut params = Vec::new();
    if !self.eat(TokenKind::CloseDelim(Delim::Paren)) {
      // self.parse_inside_delim(Delim::Paren, |this: &mut Self| {
      // return Ok(());
      // })?;
      todo!()
    };

    let return_ty = self.parse_fn_return_ty()?;

    return Ok(FnDecl { params, return_ty });
  }
  pub(crate) fn parse_fn_generics(&mut self) -> PResult<'a, Generics> {
    if !self.eat(TokenKind::Lt) {
      return Ok(Generics {
        params: Vec::new(),
        span: self.prev_token.span.shrink_to_hi(),
      });
    };

    todo!()
  }
  pub(crate) fn parse_fn_body(&mut self) -> PResult<'a, Option<Block>> {
    if self.check(TokenKind::Semicolon) {
      return Ok(None);
    };
    let lo = self.token.span;

    self.expect(TokenKind::OpenDelim(Delim::Brace))?;

    let mut statements = Vec::new();
    while !self.eat(TokenKind::CloseDelim(Delim::Brace)) {
      if self.check(TokenKind::Eof) {
        break;
      };

      let statement = self.parse_statement()?;

      if let Some(statement) = statement {
        statements.push(statement);
      } else {
        // Skip repetive ';' (if any)
        continue;
      };
    }

    return Ok(Some(Block {
      idx: NodeIdx::DUMMY,

      statements,
      span: lo.to(self.prev_token.span),
    }));
  }
  pub(crate) fn parse_fn_return_ty(&mut self) -> PResult<'a, FnReturnType> {
    if !self.eat(TokenKind::RArrow) {
      return Ok(FnReturnType::Default);
    };

    let return_ty = self.parse_ty()?;
    match return_ty.kind {
      TyKind::Tuple(items) if items.len() == 0 => {
        return Ok(FnReturnType::Default);
      }
      _ => {}
    };
    return Ok(FnReturnType::Other(return_ty));
  }
}
