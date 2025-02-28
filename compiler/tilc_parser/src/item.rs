use crate::{Parser, PathStyle, UnexpectedToken};

use tilc_ast::{
  BinOp, Block, Delim, Fn, FnDecl, FnHeader, FnReturnType, FnSig, Generics,
  Item, ItemInfo, ItemKind, NodeIdx, Param, Path, Sandyq, Statement, TokenKind,
  Ty, TyKind, Use, UseKind, Visibility, VisibilityKind,
};
use tilc_errors::{Diagnostic, Level, PResult};
use tilc_span::{kw, Identifier, Span, Symbol};


impl<'a> Parser<'a> {
  pub fn parse_sandyq(&mut self) -> PResult<'a, Sandyq> {
    let items: Vec<Item> = self.parse_until(TokenKind::Eof)?;

    return Ok(Sandyq {
      idx: NodeIdx::EMPTY,

      attributes: Vec::new(),

      items,
    });
  }
  pub(crate) fn parse_until<T: AsRef<TokenKind>>(
    &mut self,
    stopper: T,
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

    let item_info: ItemInfo = if self.eat_kw(kw::Use) {
      self.parse_use_item()?
    } else if self.check_for_fn_item() {
      self.parse_fn_item(start_span)?
    } else {
      return Ok(None);
    };

    return Ok(Some(item_info));
  }
  pub(crate) fn parse_item(&mut self) -> PResult<'a, Option<Item>> {
    let lo: Span = self.token.span;
    let vis: Visibility = self.parse_visibility()?;

    let Some((ident, kind)): Option<ItemInfo> = self.parse_item_info(lo)?
    else {
      return Ok(None);
    };

    return Ok(Some(Item {
      idx: NodeIdx::EMPTY,
      ident,
      span: lo.to(self.prev_token.span),
      kind,
      visibility: vis,
    }));
  }


  pub(crate) fn parse_ident(&mut self) -> PResult<'a, Identifier> {
    let (ident, raw): (Identifier, bool) = match self.token.ident() {
      Some((ident, raw)) => (ident, raw),
      None => {
        dbg!("{:#?}", self.token);
        let diag = UnexpectedToken {
          current_token: self.token,
          expected_token_kind: TokenKind::Identifier(Symbol::EMPTY, false),
        };
        return Err(diag.into_diag(self.dcx(), Level::Error));
      }
    };
    if !raw && ident.is_reserved() {
      todo!();
    };

    self.step();
    return Ok(ident);
  }

  pub(crate) fn parse_use_item(&mut self) -> PResult<'a, ItemInfo> {
    debug_assert!(self.prev_token.is_kw(kw::Use));
    let use_path: Use = self.parse_use_path()?;
    self.expect(TokenKind::Semicolon)?;


    return Ok((Identifier::EMPTY, ItemKind::Use(Box::new(use_path))));
    // debug_assert!(self.prev_token.is_kw(kw::Use));
    //
    // let lo: Span = self.token.span;
    // let mut ident: Identifier = Identifier::EMPTY;
    // let kind: UseKind;
    //
    // let path: Box<Path> = Box::new(self.parse_path()?);
    // if self.step_if(TokenKind::BinOp(BinOp::Star)) {
    // kind = UseKind::Everything;
    // } else if self.step_if(TokenKind::OpenDelim(Delim::Brace)) {
    // let mut idents: Vec<Identifier> = Vec::new();
    // self.parse_inside_delim(
    // Delim::Brace,
    // TokenKind::Comma,
    // |this: &mut Self| -> PResult<'a, ()> {
    // idents.push(this.parse_ident()?);
    // return Ok(());
    // },
    // )?;
    // kind = UseKind::Multiple(idents);
    // } else {
    // ident = path.segments.last().unwrap().ident;
    // .and_then(|p| Some(p.ident))
    // .unwrap_or_else(|| Identifier::EMPTY);

    // kind = UseKind::Single(ident);
    // };

    // if !self.step_if(TokenKind::Semicolon) {
    //   return Err(
    //     UnexpectedToken {
    //       current_token: self.token,
    //       expected_token_kind: TokenKind::Semicolon,
    //     }
    //     .into_diag(self.dcx(), Level::Error),
    //   );
    // };

    // return Ok((
    //   ident,
    //   ItemKind::Use(Box::new(Use {
    //     path,
    //     span: lo.to(self.prev_token.span.shrink_to_hi()),
    //     kind,
    //   })),
    // ));
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
      // self.parse_inside_delim(Delim::Paren, |this: &mut Self| {
      // return Ok(());
      // })?;
      todo!()
    };

    let return_ty: FnReturnType = self.parse_fn_return_ty()?;

    return Ok(FnDecl { params, return_ty });
  }
  pub(crate) fn parse_fn_generics(&mut self) -> PResult<'a, Generics> {
    if !self.step_if(TokenKind::Lt) {
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
    let lo: Span = self.token.span;

    // dbg!("{:#?}", self.token);
    self.expect(TokenKind::OpenDelim(Delim::Brace))?;

    let mut statements: Vec<Statement> = Vec::new();
    while !self.step_if(TokenKind::CloseDelim(Delim::Brace)) {
      if self.check(TokenKind::Eof) {
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
      span: lo.to(self.prev_token.span),
    }));
  }
  pub(crate) fn parse_fn_return_ty(&mut self) -> PResult<'a, FnReturnType> {
    if !self.step_if(&TokenKind::RArrow) {
      return Ok(FnReturnType::Default);
    };

    let return_ty: Box<Ty> = self.parse_ty()?;
    match return_ty.kind {
      TyKind::Tuple(items) if items.len() == 0 => {
        return Ok(FnReturnType::Default);
      }
      _ => {}
    };
    return Ok(FnReturnType::Other(return_ty));
  }


  pub(crate) fn parse_ty(&mut self) -> PResult<'a, Box<Ty>> {
    let mut lo: Span = self.token.span;

    let ty_kind: TyKind = if self.step_if(TokenKind::Not) {
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

  /// Assumes that prev token kind is open paranthesis
  pub(crate) fn parse_inside_delim<T, F: FnMut(&mut Self) -> PResult<'a, T>>(
    &mut self,
    delim: Delim,
    separotor: impl AsRef<TokenKind>,
    mut f: F,
  ) -> PResult<'a, T> {
    debug_assert_eq!(self.prev_token.kind, TokenKind::OpenDelim(delim));
    return loop {
      let res: PResult<'a, T> = f(self);

      if !self.step_if(&separotor) {
        if self.step_if(TokenKind::CloseDelim(delim)) {
          break res;
        };

        return Err(self.expect(&separotor).unwrap_err());
      };
      // let maybe_end = self.expect(&separotor);
      // if maybe_end.is_err() && self.step_if(TokenKind::CloseDelim(delim)) {
      // break res;
      // };
      // {PathBuf, Path}

      // if let Err(err) = self.expect(&separotor) {
      // if !self.step_if(TokenKind::CloseDelim(delim)) {
      // break Err(err);
      // };
      // };

      // if self.step_if(TokenKind::CloseDelim(delim)) {
      // break res;
      // };
    };
  }
  pub(crate) fn parse_statement(&mut self) -> PResult<'a, Option<Statement>> {
    dbg!("{:#?}", &self.token_cursor);

    Ok(None)
  }
}
