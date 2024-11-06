use tilc_ast::{Literal, LiteralKind, Token, TokenKind};
use tilc_session::ParseSession;
use tilc_span::{ModuleIdx, Pos, Span, Symbol};


struct TokenReader<'lex> {
  src: &'lex str,
  lexer: tilc_lexer::Lexer<'lex>,
  parse_session: ParseSession,

  start_post: Pos,
  pos: Pos,

  module_idx: ModuleIdx,
}
impl<'lex> TokenReader<'lex> {
  fn next_token(&mut self) -> (Token, bool) {
    let mut with_whitespace: bool = false;


    loop {
      let mut initial_str: &str = self.lexer.as_str();
      let token: tilc_lexer::Token = self.lexer.char_to_token();
      let start: Pos = self.pos;
      self.pos += Pos::new(token.len as u32);


      let kind: TokenKind = {
        use tilc_ast::Delim::*;
        // using self instead of tilc_ast? But does it really matter??
        use tilc_ast::TokenKind::*;

        match token.kind {
          tilc_lexer::TokenKind::Whitespace => {
            with_whitespace = true;
            continue;
          }
          tilc_lexer::TokenKind::LineComment => todo!(),
          tilc_lexer::TokenKind::BlockComment => todo!(),

          tilc_lexer::TokenKind::Identifier => self.identifier(start),
          tilc_lexer::TokenKind::RawIdentifier => todo!(),
          tilc_lexer::TokenKind::InvalidIdentifier => todo!(),

          tilc_lexer::TokenKind::Literal { kind, suffix_pos } => {
            let suffix_pos: Pos = start + Pos::new(suffix_pos);
            let (literal_kind, symbol): (tilc_ast::LiteralKind, Symbol) =
              self.literal(kind, start, suffix_pos);

            let suffix: Option<Symbol> =
              if suffix_pos < self.pos { todo!() } else { None };

            Literal(tilc_ast::Literal {
              kind: literal_kind,
              symbol,
              suffix,
            })
          }

          tilc_lexer::TokenKind::Lifetime => todo!(),

          tilc_lexer::TokenKind::Semicolon => Semicolon,
          tilc_lexer::TokenKind::Colon => Colon,
          tilc_lexer::TokenKind::Comma => Comma,
          tilc_lexer::TokenKind::Dot => Dot,
          tilc_lexer::TokenKind::OpenParen => OpenDelim(Paren),
          tilc_lexer::TokenKind::CloseParen => CloseDelim(Paren),
          tilc_lexer::TokenKind::OpenBrace => OpenDelim(Brace),
          tilc_lexer::TokenKind::CloseBrace => CloseDelim(Brace),
          tilc_lexer::TokenKind::OpenBracket => OpenDelim(Bracket),
          tilc_lexer::TokenKind::CloseBracket => CloseDelim(Bracket),
          tilc_lexer::TokenKind::At => At,
          tilc_lexer::TokenKind::Hashtag => Hashtag,
          tilc_lexer::TokenKind::Tilde => Tilde,
          tilc_lexer::TokenKind::Question => Question,
          tilc_lexer::TokenKind::Dollar => Dollar,
          tilc_lexer::TokenKind::Eq => Eq,
          tilc_lexer::TokenKind::Bang => Not,
          tilc_lexer::TokenKind::Lt => Lt,
          tilc_lexer::TokenKind::Gt => Gt,
          tilc_lexer::TokenKind::Minus => Minus,
          tilc_lexer::TokenKind::Plus => Plus,
          tilc_lexer::TokenKind::And => And,
          tilc_lexer::TokenKind::Or => Or,
          tilc_lexer::TokenKind::Star => Star,
          tilc_lexer::TokenKind::Slash => Slash,
          tilc_lexer::TokenKind::Caret => Caret,
          tilc_lexer::TokenKind::Percent => Percent,
          tilc_lexer::TokenKind::Unknown => {
            todo!();
          }

          tilc_lexer::TokenKind::Eof => Eof,
        }
      };
      let span: Span = Span::new(start, self.pos, 0);
      return (Token::new(kind, span), with_whitespace);
    }
  }

  fn identifier(&self, start: Pos) -> TokenKind {
    let symbol: Symbol = Symbol::intern(self.str_from(start));
    let span: Span = self.mk_span(start, self.pos);
    self.parse_session.symbol_repo.insert(symbol, span);
    return TokenKind::Identifier(symbol, false);
  }
  fn literal(
    &self,
    literal_kind: tilc_lexer::LiteralKind,
    start: Pos,
    suffix_pos: Pos,
  ) -> (tilc_ast::LiteralKind, Symbol) {
    return match literal_kind {
      tilc_lexer::LiteralKind::Int { base } => {
        todo!();
      }

      _ => todo!(),
    };
  }


  fn str_from_to(&self, start: Pos, end: Pos) -> &str {
    return &self.src[self.src_pos(start)..self.src_pos(end)];
  }
  fn str_from(&self, start: Pos) -> &str {
    return self.str_from_to(start, self.pos);
  }
  fn src_pos(&self, pos: Pos) -> usize {
    return (self.start_post - pos).into();
  }


  /// Always returns span with root context
  fn mk_span(&self, start: Pos, end: Pos) -> Span {
    return Span::new(start, end, 0);
  }
}


#[cfg(test)]
mod test {
  use super::TokenReader;
  use tilc_ast::{Delim, Token, TokenKind};
  use tilc_session::{ParseSession, SymbolRepo};
  use tilc_span::{ModuleIdx, Pos, Span};

  #[test]
  fn next_token() {
    let src: &str = "( );";

    let mut token_reader: TokenReader = TokenReader {
      src,
      lexer: tilc_lexer::Lexer::new(src),
      parse_session: ParseSession {
        edition: tilc_span::Edition::default(),
        symbol_repo: SymbolRepo::new(),
      },

      start_post: Pos::new(0),
      pos: Pos::new(0),

      module_idx: ModuleIdx::new(0),
    };

    // assert_eq!(
    //   (
    //     Token::new(TokenKind::OpenDelim(Delim::Paren), Span::new(0, 1, 0)),
    //     false
    //   ),
    //   token_reader.next_token()
    // );
    // assert_eq!(
    //   (
    //     Token::new(TokenKind::CloseDelim(Delim::Paren), Span::new(2, 3, 0)),
    //     true
    //   ),
    //   token_reader.next_token()
    // );

    // assert_eq!(
    //   (Token::new(TokenKind::Semicolon, Span::new(3, 4, 0)), false),
    //   token_reader.next_token()
    // );
  }
}
