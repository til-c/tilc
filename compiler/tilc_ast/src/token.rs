use tilc_span::{Span, Symbol};


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Literal {
  pub kind: LiteralKind,
  pub symbol: Symbol,
  pub suffix: Option<Symbol>,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LiteralKind {
  Char,
  Bool,
  Int,
  Float,
  Str,
  RawStr, // TODO: Implement for lexer first

  Error(),
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BinOp {
  /// '+'
  Plus,
  /// '-'
  Minus,
  /// '*'
  Star,
  /// '/'
  Slash,
  /// '%'
  Percent,
  /// '^'
  Caret,
  /// '&' (bitwise)
  And,
  /// '|' (bitwise)
  Or,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Delim {
  /// "()"
  Paren,
  /// "{}"
  Brace,
  /// "[]"
  Bracket,

  Empty,
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenKind {
  Identifier(Symbol, bool),
  Literal(Literal),
  Lifetime,

  /// '='
  Eq,
  /// "=="
  EqEq,
  /// '<'
  Lt,
  /// "<="
  Le,
  /// '>'
  Gt,
  /// ">="
  Ge,
  /// "&&"
  AndAnd,
  /// "||"
  OrOr,
  /// '!'
  Not,
  /// "!="
  NotEq,
  BinOp(BinOp),
  BinOpEq(BinOp),

  /// '@'
  At,
  /// '.'
  Dot,
  /// '..'
  DotDot,
  /// '...'
  DotDotDot,
  /// ','
  Comma,
  /// ':'
  Colon,
  /// ';'
  Semicolon,
  /// "::"
  Path,
  /// "->"
  RArrow,
  /// "<-"
  LArrow,
  /// '#'
  Hashtag,
  /// '~'
  Tilde,
  /// '?'
  Question,
  /// '$'
  Dollar,
  // /// '-'
  // Minus,
  // /// '+'
  // Plus,
  /// '&'
  And,
  /// '|'
  Or,
  // /// '*'
  // Star,
  // /// '/'
  // Slash,
  /// '^'
  Caret,
  /// '%'
  Percent,
  OpenDelim(Delim),
  CloseDelim(Delim),

  Unknown,
  /// End of line
  Eof,
}


pub struct Identifier {
  pub symbol: Box<str>,
  pub span: Span,
  pub raw: bool,
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Token {
  pub kind: TokenKind,
  pub span: Span,
}
impl Token {
  pub const EMPTY: Self = Self {
    kind: TokenKind::Hashtag,
    span: Span::EMPTY,
  };


  pub fn new(kind: TokenKind, span: Span) -> Self {
    return Self { kind, span };
  }


  pub fn glueable(&self, next_token: Self) -> Option<Self> {
    use crate::BinOp;
    use TokenKind::*;


    let kind: TokenKind = match self.kind {
      Eq => match next_token.kind {
        Eq => EqEq,
        _ => return None,
      },
      Lt => match next_token.kind {
        Eq => Le,
        BinOp(BinOp::Minus) => LArrow,
        _ => return None,
      },
      Gt => match next_token.kind {
        Eq => Ge,
        _ => return None,
      },
      Not => match next_token.kind {
        Eq => NotEq,
        _ => return None,
      },
      BinOp(op) => match next_token.kind {
        Eq => BinOpEq(op),
        BinOp(BinOp::And) if op == BinOp::And => AndAnd,
        BinOp(BinOp::Or) if op == BinOp::Or => OrOr,
        Gt if op == BinOp::Minus => RArrow,
        _ => return None,
      },
      Dot => match next_token.kind {
        Dot => DotDot,
        DotDot => DotDotDot,
        _ => return None,
      },
      DotDot => match next_token.kind {
        Dot => DotDotDot,
        _ => return None,
      },
      Colon => match next_token.kind {
        Colon => Path,
        _ => return None,
      },

      // TODO: Glue single quote and identifier tokens for producting lifetimes
      _ => return None,
    };


    return Some(Token::new(kind, self.span.to(next_token.span)));
  }
}
