use tilc_ast::{Token, TokenKind};
use tilc_lexer::Lexer;
use tilc_session::ParseSession;
use tilc_span::{BytePos, Pos, Span, SpanCtxt, Symbol};

pub struct TokenReader<'psess, 'lex> {
  pub src: &'lex str,
  pub lexer: Lexer<'lex>,
  pub psess: &'psess ParseSession,

  pub start_pos: BytePos,
  pub pos: BytePos,
}
impl<'psess, 'lex> TokenReader<'psess, 'lex> {
  pub fn new(
    src: &'lex str,
    lexer: tilc_lexer::Lexer<'lex>,
    psess: &'psess ParseSession,

    start_pos: BytePos,
    pos: BytePos,
  ) -> Self {
    Self {
      src,
      lexer,
      psess,

      start_pos,
      pos,
    }
  }

  pub fn next_token(&mut self) -> (Token, bool) {
    let mut with_whitespace = false;

    loop {
      let token: tilc_lexer::Token = self.lexer.char_to_token();
      let start: BytePos = self.pos;
      self.pos = self.pos + BytePos::from_usize(token.len);

      let kind: TokenKind = {
        use tilc_ast::{Delim::*, TokenKind::*};

        match token.kind {
          tilc_lexer::TokenKind::Whitespace => {
            with_whitespace = true;
            continue;
          }
          tilc_lexer::TokenKind::LineComment => continue,
          tilc_lexer::TokenKind::BlockComment => continue,

          tilc_lexer::TokenKind::Ident => self.ident(start),
          tilc_lexer::TokenKind::RawIdent => todo!(),

          tilc_lexer::TokenKind::Literal { kind, suffix_pos } => {
            let suffix_pos: BytePos = start + BytePos::from_u32(suffix_pos);
            let (literal_kind, symbol): (tilc_ast::LitKind, Symbol) =
              self.literal(kind, start, suffix_pos);

            let suffix = if suffix_pos < self.pos {
              let str: &str = self.str_from(suffix_pos);
              if str == "_" {
                todo!("empty underscore suffix");
              }

              Some(Symbol::intern(str))
            } else {
              None
            };

            Literal(tilc_ast::Lit {
              kind: literal_kind,
              symbol,
              suffix,
            })
          }

          tilc_lexer::TokenKind::Lifetime => {
            let lifetime_ident: Symbol = Symbol::intern(self.str_from(start));
            todo!();
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
          tilc_lexer::TokenKind::Bang => Bang,
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

      let span = self.mk_span(start, self.pos);
      return (Token { kind, span }, with_whitespace);
    }
  }
  fn ident(&self, start: BytePos) -> TokenKind {
    let symbol = Symbol::intern(self.str_from(start));
    let span = self.mk_span(start, self.pos);
    self.psess.symbol_repo.insert(symbol, span);
    TokenKind::Ident(symbol, false)
  }
  fn literal(
    &self,
    literal_kind: tilc_lexer::LiteralKind,
    start: BytePos,
    suffix_pos: BytePos,
  ) -> (tilc_ast::LitKind, Symbol) {
    return match literal_kind {
      tilc_lexer::LiteralKind::Int { base } => {
        let kind: tilc_ast::LitKind = tilc_ast::LitKind::Int;
        match base {
          tilc_lexer::Base::Decimal => {}

          _ => unimplemented!(),
        };

        (kind, self.symbol_from_to(start, suffix_pos))
      }
      tilc_lexer::LiteralKind::Float { base } => {
        let kind: tilc_ast::LitKind = tilc_ast::LitKind::Float;
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

      tilc_lexer::LiteralKind::Str => {
        let kind = tilc_ast::LitKind::Str;

        (kind, self.symbol_from_to(start, suffix_pos))
      }

      _ => todo!(),
    };
  }

  fn str_from_to(&self, start: BytePos, end: BytePos) -> &str {
    &self.src[self.src_pos(start)..self.src_pos(end)]
  }
  fn str_from(&self, start: BytePos) -> &str {
    self.str_from_to(start, self.pos)
  }
  fn symbol_from_to(&self, start: BytePos, end: BytePos) -> Symbol {
    Symbol::intern(self.str_from_to(start, end))
  }
  fn src_pos(&self, pos: BytePos) -> usize {
    (pos - self.start_pos).to_usize()
  }

  fn mk_span(&self, lo: BytePos, hi: BytePos) -> Span {
    Span::new(lo, hi, SpanCtxt::ROOT, None)
  }
}
