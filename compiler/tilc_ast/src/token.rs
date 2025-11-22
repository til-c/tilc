use tilc_span::{Ident, Span, Symbol};

#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub struct Token {
  pub kind: TokenKind,
  pub span: Span,
}
impl Token {
  pub const DUMMY: Self = Self {
    kind: TokenKind::Eof,
    span: Span::EMPTY,
  };

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
      Bang => match next_token.kind {
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

    Some(Token {
      kind,
      span: self.span.to(next_token.span),
    })
  }
  pub fn ident(&self) -> Option<(Ident, bool)> {
    match self.kind {
      TokenKind::Ident(name, raw) => Some((
        Ident {
          name,
          span: self.span,
        },
        raw,
      )),
      _ => None,
    }
  }
  pub fn is_kw(&self, kw: Symbol) -> bool {
    match self.kind {
      TokenKind::Ident(name, false) if name == kw => true,
      _ => false,
    }
  }
}

#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum TokenKind {
  Ident(Symbol, bool),
  Literal(Lit),
  Lifetime(Symbol, bool),

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
  Bang,
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
  /// '^'
  Caret,
  /// '%'
  Percent,

  OpenDelim(Delim),
  CloseDelim(Delim),

  /// End of line
  Eof,
  Unknown,
}

#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub struct Lit {
  pub kind: LitKind,
  pub symbol: Symbol,
  pub suffix: Option<Symbol>,
}

#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum LitKind {
  Bool,
  Int,
  Float,
  Char,
  Str,
  RawStr,
}

#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
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

#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum Delim {
  /// "()"
  Paren,
  /// "{}"
  Brace,
  /// "[]"
  Bracket,

  Empty,
}
