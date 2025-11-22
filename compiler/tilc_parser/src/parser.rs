use tilc_ast::{Delim, Spacing, Token, TokenCursor, TokenKind, TokenStream};
use tilc_error::PResult;
use tilc_session::ParseSession;
use tilc_span::Symbol;

#[derive(Debug)]
pub struct Parser<'psess> {
  pub(crate) psess: &'psess ParseSession,

  pub(crate) token: Token,
  pub(crate) prev_token: Token,
  token_spacing: Spacing,

  token_cursor: TokenCursor,

  pos: u32,
}
impl<'psess> Parser<'psess> {
  pub fn new(psess: &'psess ParseSession, token_stream: TokenStream) -> Self {
    let mut parser = Self {
      psess,

      token: Token::DUMMY,
      prev_token: Token::DUMMY,
      token_spacing: Spacing::Whitespaced,

      token_cursor: TokenCursor::new(token_stream.into_tree(), Vec::new()),

      pos: 0,
    };

    parser.step();
    parser.pos = 0;

    parser
  }

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

  pub(crate) fn check(&self, token_kind: TokenKind) -> bool {
    self.token.kind == token_kind
  }
  pub(crate) fn check_kw(&self, kw: Symbol) -> bool {
    self.token.is_kw(kw)
  }

  pub(crate) fn eat(&mut self, check: TokenKind) -> bool {
    if self.check(check) {
      self.step();
      return true;
    };

    false
  }
  pub(crate) fn eat_kw(&mut self, kw: Symbol) -> bool {
    if self.check_kw(kw) {
      self.step();
      return true;
    };

    false
  }

  pub(crate) fn expect(&mut self, token_kind: TokenKind) -> PResult<'psess, Token> {
    if self.check(token_kind) {
      self.step();
      return Ok(self.token);
    } else {
      todo!();
    };
  }
  pub(crate) fn expect_kw(&mut self, kw: Symbol) -> PResult<'psess, Token> {
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
  pub(crate) fn look_ahead_and<R, F: FnOnce(Token) -> R>(&self, n: usize, f: F) -> R {
    let token: Token = self.look_ahead(n);
    return f(token);
  }
}
