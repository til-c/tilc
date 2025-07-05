use tilc_ast::{
  Delim, Item, ItemKind, Spacing, Token, TokenCursor, TokenKind, TokenStream,
};
use tilc_error::PResult;
use tilc_session::ParseSession;
use tilc_span::{Ident, Span, Symbol};


pub(crate) type ItemInfo = (Ident, ItemKind);


pub struct Parser<'psess> {
  pub parse_session: &'psess ParseSession,

  pub token: Token,
  pub prev_token: Token,
  pub token_spacing: Spacing,

  pub token_cursor: TokenCursor,
  pub pos: u32,
}
impl<'psess> Parser<'psess> {
  pub fn new(
    parse_session: &'psess ParseSession,
    token_stream: TokenStream,
  ) -> Self {
    let mut parser = Parser {
      parse_session,

      token: Token::DUMMY,
      prev_token: Token::DUMMY,
      token_spacing: Spacing::Whitespaced,

      token_cursor: TokenCursor::new(token_stream.into_tree(), Vec::new()),
      pos: 0,
    };


    // Inits parser.token and resets the position afterwards
    parser.step();
    parser.pos = 0;

    return parser;
  }
}
impl<'a> Parser<'a> {
  pub(crate) fn step(&mut self) {
    let (token, spacing) = self.token_cursor.step();
    self.pos += 1;


    debug_assert!(!matches!(
      token.kind,
      TokenKind::OpenDelim(Delim::Empty) | TokenKind::CloseDelim(Delim::Empty)
    ));
    self.prev_token = std::mem::replace(&mut self.token, token);
    self.token_spacing = spacing;
  }

  pub(crate) fn step_if(&mut self, check: TokenKind) -> bool {
    if self.check(check) {
      self.step();
      return true;
    };

    return false;
  }


  pub(crate) fn check(&self, token_kind: TokenKind) -> bool {
    return self.token.kind == token_kind;
  }
  pub(crate) fn check_kw(&self, kw: Symbol) -> bool {
    return self.token.is_kw(kw);
  }
  pub(crate) fn check_kw_ahead(&self, n: usize, kw: Symbol) -> bool {
    let nth_token = self.look_ahead(n);
    return nth_token.is_kw(kw);
  }

  pub(crate) fn expect(&mut self, token_kind: TokenKind) -> PResult<'a, Token> {
    if self.check(token_kind) {
      self.step();
      return Ok(self.token);
    } else {
      todo!();
      // let diag: Diag<'a> = self.dcx().create_error(UnexpectedToken {
      //   expected_token_kind: *token_kind,
      //   current_token: self.token,
      // });
      // return Err(diag);
    };
  }

  pub(crate) fn eat_kw(&mut self, kw: Symbol) -> bool {
    if self.check_kw(kw) {
      self.step();
      return true;
    };

    return false;
  }
  pub(crate) fn expect_kw(&mut self, kw: Symbol) -> PResult<'a, Token> {
    if self.check_kw(kw) {
      self.step();
      return Ok(self.prev_token);
    } else {
      todo!()
    };
  }

  pub(crate) fn look_ahead(&self, n: usize) -> Token {
    let mut token_cursor = self.token_cursor.clone();
    let mut token = self.token;

    for _ in 0..n {
      token = token_cursor.step().0;
    }

    return token;
  }
  pub(crate) fn look_ahead_and<R, F: FnOnce(Token) -> R>(
    &self,
    n: usize,
    f: F,
  ) -> R {
    let token: Token = self.look_ahead(n);
    return f(token);
  }
}

impl<'a> Parser<'a> {
  pub(crate) fn parse_until(
    &mut self,
    stopper: TokenKind,
  ) -> PResult<'a, (Vec<Item>, Span)> {
    let mut items = Vec::new();
    let lo = self.token.span;

    loop {
      let Some(item): Option<Item> = self.parse_item()? else {
        break;
      };
      items.push(item);
    }

    let span = lo.to(self.prev_token.span);
    if !self.step_if(stopper) {
      todo!();
    };


    return Ok((items, span));
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
    return Ok(ident);
  }
}
