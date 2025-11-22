#[derive(Debug)]
#[derive(PartialEq)]
pub struct Token {
  pub kind: TokenKind,
  pub len: usize,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum TokenKind {
  Whitespace,
  LineComment,
  BlockComment,

  Ident,
  RawIdent,

  Literal {
    kind: LiteralKind,
    suffix_pos: u32,
  },

  Lifetime,

  /// ';'
  Semicolon,
  /// ':'
  Colon,
  /// ','
  Comma,
  /// '.'
  Dot,
  /// '('
  OpenParen,
  /// ')'
  CloseParen,
  /// '{'
  OpenBrace,
  /// '}'
  CloseBrace,
  /// '['
  OpenBracket,
  /// ']'
  CloseBracket,
  /// '@'
  At,
  /// '#'
  Hashtag,
  /// '~'
  Tilde,
  /// '?'
  Question,
  /// '$'
  Dollar,
  /// '='
  Eq,
  /// '!'
  Bang,
  /// '<'
  Lt,
  /// '>'
  Gt,
  /// '-'
  Minus,
  /// '+'
  Plus,
  /// '&'
  And,
  /// '|'
  Or,
  /// '*'
  Star,
  /// '/'
  Slash,
  /// '^'
  Caret,
  /// '%'
  Percent,

  Eof,
  Unknown,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Base {
  /// Prefix: "0e"
  Binary,

  /// Prefix: "0s",
  Octal,

  /// No prefix
  Decimal,

  /// Prefix: "0o"
  Hexadecimal,
}
#[derive(Debug)]
#[derive(PartialEq)]
pub enum LiteralKind {
  Int { base: Base },
  Float { base: Base },

  Char,
  Str,
  Byte,
  RawStr,
}
