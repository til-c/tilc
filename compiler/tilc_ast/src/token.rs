use tilc_span::Span;


#[derive(Debug, PartialEq)]
pub struct Literal {
  pub kind: LiteralKind,
}
#[derive(Debug, PartialEq)]
pub enum LiteralKind {
  Char,
  Bool,
  Int,
  Float,
  Str,
  RawStr, // TODO: Implement for lexer first
}
#[derive(Debug, PartialEq)]
pub enum BinOp {
  /// +
  Plus,
  /// -
  Minus,
  /// *
  Star,
  /// /
  Slash,
  /// %
  Percent,
  /// ^
  Caret,
  /// & (bitwise)
  And,
  /// | (bitwise)
  Or,
}
#[derive(Debug, PartialEq)]
pub enum Delim {
  /// ()
  Paren,
  /// {}
  Brace,
  /// []
  Bracket,
}


#[derive(Debug, PartialEq)]
pub enum TokenKind {
  Identifier,
  Literal(Literal),
  Lifetime,

  /// =
  Eq,
  /// ==
  EqEq,
  /// <
  Lt,
  /// <=
  Le,
  /// >
  Gt,
  /// >=
  Ge,
  /// &&
  AndAnd,
  /// ||
  OrOr,
  /// !
  Not,
  /// ~
  Tilde,
  BinOp(BinOp),

  /// @
  At,
  /// .
  Dot,
  /// ,
  Comma,
  /// :
  Semi,
  /// ;
  Semicolon,
  /// ::
  Path,
  /// ->
  RArrow,
  /// <-
  LArrow,
  /// #
  Hash,
  /// ?
  Question,
  OpenDelim(Delim),
  CloseDelim(Delim),

  /// End of line
  Eof,
}


pub struct Identifier {
  pub symbol: Box<str>,
  pub span: Span,
  pub raw: bool,
}
