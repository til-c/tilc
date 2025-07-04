#[derive(Debug, PartialEq)]
pub struct Token {
  pub kind: TokenKind,
  pub len: usize,
}
impl Token {
  pub(crate) fn new(kind: TokenKind, len: usize) -> Self {
    return Self { kind, len };
  }
}


#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
pub enum LiteralKind {
  Int { base: Base },
  Float { base: Base },

  Char,
  Str,
  Byte,
  RawStr,
}
