use std::str::Chars;

use crate::token::{
  Base, LiteralKind, Token,
  TokenKind::{self},
};


const EOF_CHAR: char = '\0';
const RAW_CHAR: char = 's';


fn is_whitespace(char: char) -> bool {
  return match char {
    ' ' | '\t' | '\r' | '\n' => true,

    _ => false,
  };
}
fn is_identifier_start(char: char) -> bool {
  return match char {
    c if c.is_ascii_alphabetic() || c == '_' => true,

    _ => false,
  };
}
fn is_identifier(char: char) -> bool {
  return match char {
    c if c.is_ascii_alphanumeric() || c == '_' => true,

    _ => false,
  };
}


pub struct Lexer<'a> {
  chars: Chars<'a>,
  len_left: usize,
}
impl<'a> Lexer<'a> {
  pub fn new(str: &'a str) -> Self {
    return Self {
      chars: str.chars(),
      len_left: str.len(),
    };
  }

  pub fn as_str(&self) -> &'a str {
    return self.chars.as_str();
  }


  fn peek(&self) -> char {
    return match self.chars.clone().next() {
      Some(char) => char,
      None => EOF_CHAR,
    };
  }
  fn nth(&self, n: u32) -> char {
    debug_assert_ne!(n, 0);
    let mut chars: Chars<'_> = self.chars.clone();

    for _ in 0..n - 1 {
      chars.next();
    }

    return chars.next().unwrap_or(EOF_CHAR);
  }
  fn step(&mut self) -> char {
    return self.chars.next().unwrap_or(EOF_CHAR);
  }
  fn consume(&mut self, mut until: impl FnMut(char) -> bool) {
    while until(self.peek()) {
      self.step();
    }
  }
  fn current_token_len(&self) -> usize {
    return self.len_left - self.chars.as_str().len();
  }
  fn reset_len(&mut self) {
    self.len_left = self.chars.as_str().len();
  }


  fn whitespace(&mut self) -> TokenKind {
    self.consume(is_whitespace);
    return TokenKind::Whitespace;
  }
  /// Assumes that previous char satisfies identifier start char
  fn identifier(&mut self) -> TokenKind {
    self.consume(is_identifier);

    return TokenKind::Identifier;
  }
  /// Assumes that previous char satisfies identifier start char
  fn raw_identifier(&mut self) -> TokenKind {
    todo!();
  }
  fn raw_string(&mut self) -> TokenKind {
    todo!();
  }
  fn is_digit(&mut self) -> bool {
    let mut containt_digits: bool = false;
    loop {
      match self.peek() {
        '_' => {
          self.step();
        }
        '0'..='9' => {
          containt_digits = true;
          self.step();
        }

        '.' | 'E' => {}

        _ => break,
      };
    }

    return containt_digits;
  }
  fn number_kind(&mut self, current_char: char) -> LiteralKind {
    let mut base: Base = Base::Decimal;


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

        '0'..='9' | '_' => {
          self.is_digit();
        }

        _ => return LiteralKind::Int { base },
      };
    } else {
      self.is_digit();
    };

    // TODO: Handle flaoting point number
    return match self.peek() {
      '.' if self.nth(2) != '.' => {
        todo!()
      }

      _ => LiteralKind::Int {
        base: Base::Decimal,
      },
    };
  }
  fn line_comment(&mut self) -> TokenKind {
    todo!();
  }
  fn block_comment(&mut self) -> TokenKind {
    todo!();
  }
  fn char_literal_or_lifetime(&mut self) -> TokenKind {
    debug_assert_eq!(self.peek(), '\'');
    todo!();
  }


  pub fn char_to_token(&mut self) -> Token {
    use TokenKind::*;

    let current_char: char = match self.step() {
      '\0' => return Token::new(Eof, 0),
      char => char,
    };


    let token_kind: TokenKind = match current_char {
      char if is_whitespace(char) => self.whitespace(),

      '/' => match self.peek() {
        '/' => self.line_comment(),
        '*' => self.block_comment(),

        _ => Slash,
      },

      RAW_CHAR => match (self.peek(), self.nth(2)) {
        ('#', char2) if is_identifier_start(char2) => self.raw_identifier(),
        ('#', '\"') => self.raw_string(),

        _ => self.identifier(),
      },

      char if is_identifier_start(char) => self.identifier(),

      char @ '0'..='9' => {
        let kind: LiteralKind = self.number_kind(char);
        let suffix_pos: u32 = self.current_token_len() as u32;
        // Consumes suffix if applicable
        self.consume(is_identifier);

        Literal { kind, suffix_pos }
      }

      '\'' => self.char_literal_or_lifetime(),

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
      '^' => Caret,
      '%' => Percent,

      _ => Unknown,
    };

    let token: Token = Token::new(token_kind, self.current_token_len());
    self.reset_len();
    return token;
  }
}


#[cfg(test)]
mod test {
  use crate::{
    lexer::{Lexer, EOF_CHAR},
    token::{Base, LiteralKind, Token, TokenKind},
  };


  #[test]
  fn peek() {
    assert_eq!(Lexer::new("a").peek(), 'a');
    assert_eq!(Lexer::new("0").peek(), '0');
    assert_eq!(Lexer::new("?").peek(), '?');
    assert_eq!(Lexer::new("\\").peek(), '\\');
    assert_eq!(Lexer::new("").peek(), '\0');
  }
  #[test]
  fn nth() {
    let mut l: Lexer<'_> = Lexer::new("fx basty()");


    assert_eq!(l.nth(2), 'x');
    assert_eq!(l.nth(3), ' ');
    assert_eq!(l.nth(4), 'b');
    assert_eq!(l.nth(11), '\0');
  }
  #[test]
  fn step() {
    let mut l: Lexer<'_> = Lexer::new("fx basty()");


    assert_eq!(l.step(), 'f');
    assert_eq!(l.step(), 'x');
    assert_eq!(l.step(), ' ');
    assert_eq!(l.step(), 'b');
    assert_eq!(l.step(), 'a');
    assert_eq!(l.step(), 's');
    assert_eq!(l.step(), 't');
    assert_eq!(l.step(), 'y');
    assert_eq!(l.step(), '(');
    assert_eq!(l.step(), ')');
    assert_eq!(l.step(), EOF_CHAR);
  }


  #[test]
  fn char_to_token() {
    let mut l: Lexer<'_> = Lexer::new("fx basty() -> i32 {qaitar 0_i32;}");


    assert_eq!(l.char_to_token(), Token::new(TokenKind::Identifier, 2)); // fx
    assert_eq!(l.char_to_token(), Token::new(TokenKind::Whitespace, 1)); // ' '
    assert_eq!(l.char_to_token(), Token::new(TokenKind::Identifier, 5)); // basty
    assert_eq!(l.char_to_token(), Token::new(TokenKind::OpenParen, 1)); // (
    assert_eq!(l.char_to_token(), Token::new(TokenKind::CloseParen, 1)); // )
    assert_eq!(l.char_to_token(), Token::new(TokenKind::Whitespace, 1)); // ' '
    assert_eq!(l.char_to_token(), Token::new(TokenKind::Minus, 1)); // -
    assert_eq!(l.char_to_token(), Token::new(TokenKind::Gt, 1)); // >
    assert_eq!(l.char_to_token(), Token::new(TokenKind::Whitespace, 1)); // ' '
    assert_eq!(l.char_to_token(), Token::new(TokenKind::Identifier, 3)); // i32
    assert_eq!(l.char_to_token(), Token::new(TokenKind::Whitespace, 1)); // ' '
    assert_eq!(l.char_to_token(), Token::new(TokenKind::OpenBrace, 1)); // {
    assert_eq!(l.char_to_token(), Token::new(TokenKind::Identifier, 6)); // qaitar
    assert_eq!(l.char_to_token(), Token::new(TokenKind::Whitespace, 1)); // ' '
    assert_eq!(
      l.char_to_token(),
      Token::new(
        TokenKind::Literal {
          kind: LiteralKind::Int {
            base: Base::Decimal
          },
          suffix_pos: 2
        },
        5
      )
    ); // 0_i32

    assert_eq!(l.char_to_token(), Token::new(TokenKind::Semicolon, 1)); // ;
    assert_eq!(l.char_to_token(), Token::new(TokenKind::CloseBrace, 1)); // }
    assert_eq!(l.char_to_token(), Token::new(TokenKind::Eof, 0)); // '\0'
  }
}
