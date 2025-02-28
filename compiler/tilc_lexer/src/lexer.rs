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
  pub fn new(src: &'a str) -> Self {
    return Self {
      chars: src.chars(),
      len_left: src.len(),
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

    for _ in 0..(n - 1) {
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
    self.consume(is_identifier);

    return TokenKind::RawIdentifier;
  }
  fn raw_string(&mut self) -> TokenKind {
    debug_assert_eq!(self.step(), '#');
    debug_assert_eq!(self.step(), '"');
    while self.step() != '"' {}

    return TokenKind::Literal {
      kind: LiteralKind::RawString,
      suffix_pos: self.current_token_len() as u32,
    };
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
    debug_assert_eq!(self.step(), '/');

    loop {
      if self.step() == '\n' {
        break;
      };
    }

    return TokenKind::LineComment;
  }
  fn block_comment(&mut self) -> TokenKind {
    debug_assert_eq!(self.step(), '*');

    loop {
      let next_char: char = self.step();

      match next_char {
        '*' => match self.peek() {
          '/' => return TokenKind::BlockComment,
          _ => continue,
        },

        _ => continue,
      }
    }
  }
  fn char_literal_or_lifetime(&mut self) -> TokenKind {
    let mut is_char_literal: bool = false;
    while !is_whitespace(self.peek()) || self.peek() == ' ' {
      if self.peek() == '\'' {
        is_char_literal = true;
      };
      self.step();

      if is_char_literal {
        break;
      };
    }

    println!("{}", self.as_str());
    if is_char_literal {
      return TokenKind::Literal {
        kind: LiteralKind::Char,
        suffix_pos: self.current_token_len() as u32,
      };
    } else {
      return TokenKind::Lifetime;
    };
  }

  fn terminated_string(&mut self) -> bool {
    loop {
      match self.step() {
        '\"' => return true,
        '\\' if self.peek() == '\"' => {
          self.step();
        }
        '\0' => return false,

        _ => {}
      };
    }
  }
  /// Assumes previous char is '"'
  fn string_literal(&mut self) -> TokenKind {
    let terminated = self.terminated_string();
    if !terminated {
      panic!();
    }

    return TokenKind::Literal {
      kind: LiteralKind::Str,
      suffix_pos: self.current_token_len() as u32,
    };
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

      /*
        Must be before
        [`char if is_identifier_start(char) => self.identifier(),`]
      */
      RAW_CHAR => match (self.peek(), self.nth(2)) {
        ('#', next) if is_identifier_start(next) => self.raw_identifier(),
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

      '\"' => self.string_literal(),

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

    let token: Token = Token::new(token_kind, self.current_token_len());
    self.reset_len();
    return token;
  }
}
