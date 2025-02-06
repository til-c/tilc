use tilc_ast::{Token, TokenKind};
use tilc_session::ParseSession;
use tilc_span::{BytePos, Pos, Span, SpanContext, Symbol};


pub struct TokenReader<'psess, 'lex> {
  src: &'lex str,
  lexer: tilc_lexer::Lexer<'lex>,
  parse_session: &'psess ParseSession,

  start_pos: BytePos,
  pos: BytePos,
}
impl<'psess, 'lex> TokenReader<'psess, 'lex> {
  pub fn new(
    src: &'lex str,
    lexer: tilc_lexer::Lexer<'lex>,
    parse_session: &'psess ParseSession,
    start_pos: BytePos,
    pos: BytePos,
  ) -> Self {
    return Self {
      src,

      lexer,
      parse_session,

      start_pos,
      pos,
    };
  }

  pub fn next_token(&mut self) -> (Token, bool) {
    let mut with_whitespace: bool = false;


    loop {
      let token: tilc_lexer::Token = self.lexer.char_to_token();
      let start: BytePos = self.pos;
      self.pos += BytePos::from_usize(token.len);


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
            let suffix_pos: BytePos = start + BytePos::from_u32(suffix_pos);
            let (literal_kind, symbol): (tilc_ast::LiteralKind, Symbol) =
              self.literal(kind, start, suffix_pos);

            let suffix: Option<Symbol> = if suffix_pos < self.pos {
              let str: &str = self.str_from(suffix_pos);
              if str == "_" {
                todo!("empty underscore suffix")
              }

              Some(Symbol::intern(str))
            } else {
              None
            };

            Literal(tilc_ast::Literal {
              kind: literal_kind,
              symbol,
              suffix,
            })
          }

          tilc_lexer::TokenKind::Lifetime => {
            // 'a
            let lifetime_ident: Symbol = Symbol::intern(self.str_from(start));
            println!("{:?}", lifetime_ident);
            todo!()
          }

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
          tilc_lexer::TokenKind::Minus => BinOp(tilc_ast::BinOp::Minus),
          tilc_lexer::TokenKind::Plus => BinOp(tilc_ast::BinOp::Plus),
          tilc_lexer::TokenKind::And => BinOp(tilc_ast::BinOp::And),
          tilc_lexer::TokenKind::Or => BinOp(tilc_ast::BinOp::Or),
          tilc_lexer::TokenKind::Star => BinOp(tilc_ast::BinOp::Star),
          tilc_lexer::TokenKind::Slash => BinOp(tilc_ast::BinOp::Slash),
          tilc_lexer::TokenKind::Caret => BinOp(tilc_ast::BinOp::Caret),
          tilc_lexer::TokenKind::Percent => BinOp(tilc_ast::BinOp::Percent),
          tilc_lexer::TokenKind::Unknown => todo!(),

          tilc_lexer::TokenKind::Eof => Eof,
        }
      };


      let span: Span = self.mk_span(start, self.pos);
      return (Token::new(kind, span), with_whitespace);
    }
  }

  fn identifier(&self, start: BytePos) -> TokenKind {
    let symbol: Symbol = Symbol::intern(self.str_from(start));
    let span: Span = self.mk_span(start, self.pos);
    self.parse_session.symbol_repo.insert(symbol, span);
    return TokenKind::Identifier(symbol, false);
  }
  fn literal(
    &self,
    literal_kind: tilc_lexer::LiteralKind,
    start: BytePos,
    suffix_pos: BytePos,
  ) -> (tilc_ast::LiteralKind, Symbol) {
    return match literal_kind {
      tilc_lexer::LiteralKind::Int { base } => {
        let kind: tilc_ast::LiteralKind = tilc_ast::LiteralKind::Int;
        match base {
          tilc_lexer::Base::Decimal => {}

          _ => unimplemented!(),
        };


        (kind, self.symbol_from_to(start, suffix_pos))
      }
      tilc_lexer::LiteralKind::Float { base } => {
        let kind: tilc_ast::LiteralKind = tilc_ast::LiteralKind::Float;
        let base: Option<&str> = match base {
          tilc_lexer::Base::Hexadecimal => Some("hexadecimal"),
          tilc_lexer::Base::Octal => Some("ocatl"),
          tilc_lexer::Base::Binary => Some("binary"),

          _ => None,
        };
        if let Some(base) = base {
          panic!("Unsopported base {} for float number", base);
        };


        (kind, self.symbol_from_to(start, suffix_pos))
      }

      _ => todo!(),
    };
  }


  fn str_from_to(&self, start: BytePos, end: BytePos) -> &str {
    return &self.src[self.src_pos(start)..self.src_pos(end)];
  }
  fn str_from(&self, start: BytePos) -> &str {
    return self.str_from_to(start, self.pos);
  }
  fn symbol_from_to(&self, start: BytePos, end: BytePos) -> Symbol {
    return Symbol::intern(self.str_from_to(start, end));
  }
  fn src_pos(&self, pos: BytePos) -> usize {
    return (pos - self.start_pos).to_usize();
  }


  /// Always returns span with 0 ctxt
  fn mk_span(&self, lo: BytePos, hi: BytePos) -> Span {
    return Span::new(lo, hi, SpanContext::ROOT, None);
  }
}
