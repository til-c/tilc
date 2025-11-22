use std::str::Chars;

use crate::{Base, LiteralKind, TokenKind, token::Token};

const EOF_CHAR: char = '\0';
const RAW_CHAR: char = 's';

const fn is_whitespace(ch: char) -> bool {
  match ch {
    ' ' | '\t' | '\r' | '\n' => true,

    _ => false,
  }
}
const fn is_ident_start(ch: char) -> bool {
  match ch {
    _ if ch.is_ascii_alphabetic() || ch == '_' => true,

    _ => false,
  }
}
const fn is_ident(ch: char) -> bool {
  match ch {
    _ if ch.is_ascii_alphanumeric() || ch == '_' => true,

    _ => false,
  }
}

pub struct Lexer<'a> {
  chars: Chars<'a>,
  len_left: usize,
}
impl<'a> Lexer<'a> {
  pub fn new(src: &'a str) -> Self {
    Self {
      chars: src.chars(),
      len_left: src.len(),
    }
  }

  pub fn char_to_token(&mut self) -> Token {
    use crate::TokenKind::*;

    let current_char = match self.step() {
      '\0' => return Token { kind: Eof, len: 0 },
      ch => ch,
    };

    let token_kind = match current_char {
      ch if is_whitespace(ch) => self.whitespace(),

      '/' => match self.peek() {
        '/' => self.line_comment(),
        '*' => self.block_comment(),

        _ => Slash,
      },

      RAW_CHAR => match (self.peek(), self.nth(2)) {
        ('#', next) if is_ident_start(next) => self.raw_ident(),
        ('#', '\"') => self.raw_str(),

        _ => self.ident(),
      },

      ch if is_ident_start(ch) => self.ident(),

      ch @ '0'..='9' => {
        let kind = self.number_kind(ch);
        let suffix_pos = self.current_token_len() as u32;
        self.consume(is_ident);

        Literal { kind, suffix_pos }
      }

      '\'' => self.char_literal_or_lifetime(),

      '\"' => todo!(),

      ';' => Semicolon,
      ':' => Colon,
      ',' => Comma,
      '.' => Dot,
      '(' => OpenParen,
      ')' => CloseParen,
      '{' => OpenBrace,
      '}' => CloseBrace,
      '[' => OpenBracket,
      ']' => CloseBracket,
      '@' => At,
      '#' => Hashtag,
      '?' => Question,
      '$' => Dollar,
      '=' => Eq,
      '!' => Bang,
      '<' => Lt,
      '>' => Gt,
      '-' => Minus,
      '+' => Plus,
      '&' => And,
      '|' => Or,
      '*' => Star,
      '^' => Caret,
      '%' => Percent,

      _ => Unknown,
    };
    let token = Token {
      kind: token_kind,
      len: self.current_token_len(),
    };
    self.reset_len();

    return token;
  }

  fn as_str(&self) -> &'a str {
    return self.chars.as_str();
  }
  fn current_token_len(&self) -> usize {
    return self.len_left - self.as_str().len();
  }
  fn reset_len(&mut self) {
    self.len_left = self.as_str().len();
  }

  fn nth(&self, n: u32) -> char {
    debug_assert_ne!(n, 0);
    let mut chars = self.chars.clone();
    chars.nth(n as usize).unwrap_or_else(|| EOF_CHAR)
  }
  fn peek(&self) -> char {
    match self.chars.clone().next() {
      Some(ch) => ch,
      None => EOF_CHAR,
    }
  }
  fn step(&mut self) -> char {
    self.chars.next().unwrap_or_else(|| EOF_CHAR)
  }
  fn consume<F>(&mut self, mut until: F)
  where
    F: FnMut(char) -> bool, {
    while until(self.peek()) {
      self.step();
    }
  }

  fn whitespace(&mut self) -> TokenKind {
    self.consume(is_whitespace);
    TokenKind::Whitespace
  }
  fn ident(&mut self) -> TokenKind {
    self.consume(is_ident);
    TokenKind::Ident
  }
  fn raw_ident(&mut self) -> TokenKind {
    self.consume(is_ident);
    TokenKind::RawIdent
  }
  fn raw_str(&mut self) -> TokenKind {
    debug_assert_eq!(self.step(), '#');
    debug_assert_eq!(self.step(), '\"');
    while self.step() != '"' {}

    TokenKind::Literal {
      kind: LiteralKind::RawStr,
      suffix_pos: self.current_token_len() as u32,
    }
  }
  fn is_digit(&mut self) -> bool {
    let mut contains_digits = false;
    loop {
      match self.peek() {
        '_' => {
          self.step();
        }

        '0'..='9' => {
          contains_digits = true;
          self.step();
        }

        '.' | 'E' => {}

        _ => break,
      };
    }

    return contains_digits;
  }
  fn number_kind(&mut self, current_char: char) -> LiteralKind {
    let mut base = Base::Decimal;

    if current_char == '0' {
      match self.peek() {
        'e' => {
          base = Base::Binary;
          self.step();
          if !self.is_digit() {
            return LiteralKind::Int { base };
          };
        }
        's' => {
          base = Base::Octal;
          self.step();
          if !self.is_digit() {
            return LiteralKind::Int { base };
          };
        }

        _ => return LiteralKind::Int { base },
      };
    } else {
      self.is_digit();
    };

    match self.peek() {
      '.' if self.nth(2) != '.' => {
        todo!();
      }

      _ => LiteralKind::Int {
        base: Base::Decimal,
      },
    }
  }
  fn line_comment(&mut self) -> TokenKind {
    debug_assert_eq!(self.step(), '/');

    loop {
      if self.step() == '\n' {
        break TokenKind::LineComment;
      };
    }
  }
  fn block_comment(&mut self) -> TokenKind {
    debug_assert_eq!(self.step(), '*');

    loop {
      match self.step() {
        '*' => match self.peek() {
          '/' => break TokenKind::BlockComment,
          _ => continue,
        },

        _ => continue,
      }
    }
  }
  fn char_literal_or_lifetime(&mut self) -> TokenKind {
    let mut is_char_literal = false;
    while !is_whitespace(self.peek()) || self.peek() == ' ' {
      if self.peek() == '\'' {
        is_char_literal = true;
      };
      self.step();

      if is_char_literal {
        break;
      };
    }

    if is_char_literal {
      return TokenKind::Literal {
        kind: LiteralKind::Char,
        suffix_pos: self.current_token_len() as u32,
      };
    } else {
      return TokenKind::Lifetime;
    };
  }
}
