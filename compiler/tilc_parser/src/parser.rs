use tilc_ast::{Delim, Spacing, Token, TokenCursor, TokenKind, TokenStream};
use tilc_errors::{Diag, DiagCtxtHandle, PResult};
use tilc_session::ParseSession;
use tilc_span::Symbol;

use crate::UnexpectedToken;


#[derive(Debug)]
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
    let mut parser: Parser<'_> = Parser {
      parse_session,

      token: Token::EMPTY,
      prev_token: Token::EMPTY,
      token_spacing: Spacing::Whitespaced,

      token_cursor: TokenCursor::new(token_stream.into_tree(), Vec::new()),
      pos: 0,
    };


    // Inits parser.token and resets the position afterwards
    parser.step();
    parser.pos = 0;

    return parser;
  }

  pub fn dcx(&self) -> DiagCtxtHandle<'psess> {
    return self.parse_session.dcx();
  }
}
impl<'a> Parser<'a> {
  pub(crate) fn step(&mut self) {
    let (token, spacing): (Token, Spacing) = self.token_cursor.step();
    self.pos += 1;


    debug_assert!(!matches!(
      token.kind,
      TokenKind::OpenDelim(Delim::Empty) | TokenKind::CloseDelim(Delim::Empty)
    ));
    // std::mem::swap(&mut self.token, &mut self.prev_token);
    // self.token = token;
    self.prev_token = std::mem::replace(&mut self.token, token);
    self.token_spacing = spacing;
  }
  pub(crate) fn step_if<T: AsRef<TokenKind>>(&mut self, check: T) -> bool {
    if self.check(check) {
      self.step();
      return true;
    };

    return false;
  }

  pub(crate) fn check<T: AsRef<TokenKind>>(&self, token_kind: T) -> bool {
    return self.token.kind == *token_kind.as_ref();
  }
  pub(crate) fn check_kw<S: AsRef<Symbol>>(&self, kw: S) -> bool {
    return self.token.is_kw(kw);
  }
  pub(crate) fn check_kw_ahead<S: AsRef<Symbol>>(
    &self,
    n: usize,
    kw: S,
  ) -> bool {
    let nth_token: Token = self.look_ahead(n);
    return nth_token.is_kw(kw);
  }

  pub(crate) fn expect<T: AsRef<TokenKind>>(
    &mut self,
    token_kind: T,
  ) -> PResult<'a, Token> {
    let token_kind: &TokenKind = token_kind.as_ref();

    if self.check(token_kind) {
      self.step();
      return Ok(self.token);
    } else {
      let diag: Diag<'a> = self.dcx().create_error(UnexpectedToken {
        expected_token_kind: *token_kind,
        current_token: self.token,
      });
      return Err(diag);
    };
  }
  pub(crate) fn expect_any_of<T: AsRef<[TokenKind]>>(
    &mut self,
    expectations: T,
  ) -> PResult<'a, Token> {
    let expectations: &[TokenKind] = expectations.as_ref();

    if expectations.contains(&self.token.kind) {
      self.step();
      return Ok(self.prev_token);
    } else {
      todo!()
    };
  }

  pub(crate) fn eat<T: AsRef<TokenKind>>(&mut self, token_kind: T) -> bool {
    if self.check(token_kind) {
      self.step();
      return true;
    };

    return false;
  }
  pub(crate) fn eat_kw<S: AsRef<Symbol>>(&mut self, kw: S) -> bool {
    if self.check_kw(kw) {
      self.step();
      return true;
    };

    return false;
  }

  pub(crate) fn look_ahead(&self, n: usize) -> Token {
    let mut token_cursor: TokenCursor = self.token_cursor.clone();
    let mut token: Token = self.token;

    for _ in 0..n {
      token = token_cursor.step().0;
    }

    return token;
  }
}
