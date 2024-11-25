use tilc_ast::{Delim, Spacing, Token, TokenCursor, TokenKind, TokenStream};
use tilc_session::ParseSession;


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
    let mut parser: Parser<'psess> = Parser {
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


  pub fn step(&mut self) {
    let (token, spacing): (Token, Spacing) = self.token_cursor.step();
    self.pos += 1;


    debug_assert!(!matches!(
      token.kind,
      TokenKind::OpenDelim(Delim::Empty) | TokenKind::CloseDelim(Delim::Empty)
    ));
    std::mem::swap(&mut self.token, &mut self.prev_token);
    // self.prev_token = self.token.clone();
    self.token = token;
    self.token_spacing = spacing;
  }
}
