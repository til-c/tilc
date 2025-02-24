use std::rc::Rc;


use tilc_span::Span;


use crate::{Delim, Token, TokenKind};

#[derive(Clone, Debug)]
pub struct TokenCursor {
  token_tree_cursor: TokenTreeCursor,

  stack: Vec<(TokenTreeCursor, DelimSpan, DelimSpacing, Delim)>,
}
impl TokenCursor {
  pub fn new(
    ttc: TokenTreeCursor,
    stack: Vec<(TokenTreeCursor, DelimSpan, DelimSpacing, Delim)>,
  ) -> Self {
    return Self {
      token_tree_cursor: ttc,
      stack,
    };
  }
  pub fn step(&mut self) -> (Token, Spacing) {
    loop {
      if let Some(tt) = self.token_tree_cursor.step() {
        match tt {
          &TokenTree::Token(token, spacing) => {
            return (token, spacing);
          }

          &TokenTree::Delimited(span, spacing, delim, ref t_stream) => {
            let ttc: TokenTreeCursor = t_stream.clone().into_tree();
            self.stack.push((
              self.token_tree_cursor.clone(),
              span,
              spacing,
              delim,
            ));
            self.token_tree_cursor = ttc;

            if delim != Delim::Empty {
              return (
                Token::new(TokenKind::OpenDelim(delim), span.start),
                spacing.start,
              );
            }
          }
        };
      } else if let Some((ttc, span, spacing, delim)) = self.stack.pop() {
        self.token_tree_cursor = ttc;

        if delim != Delim::Empty {
          return (
            Token::new(TokenKind::CloseDelim(delim), span.end),
            spacing.end,
          );
        }
      } else {
        return (Token::new(TokenKind::Eof, Span::EMPTY), Spacing::Sticked);
      }
    }
  }
}
#[derive(Clone, Debug)]
pub struct TokenTreeCursor {
  stream: TokenStream,
  idx: usize,
}
impl TokenTreeCursor {
  pub fn new(stream: TokenStream) -> Self {
    return Self { stream, idx: 0 };
  }

  pub fn step(&mut self) -> Option<&TokenTree> {
    return self.stream.0.get(self.idx).map(|token_tree: &TokenTree| {
      self.idx += 1;
      token_tree
    });
  }
  pub fn look(&self, n: usize) -> Option<&TokenTree> {
    return self
      .stream
      .0
      .get(self.idx + n)
      .map(|token_tree: &TokenTree| token_tree);
  }
}

#[derive(Clone, Debug)]
pub enum TokenTree {
  /// Single token and spacing info
  Token(Token, Spacing),

  /// Sequence of tokens
  Delimited(DelimSpan, DelimSpacing, Delim, TokenStream),
}
impl TokenTree {
  fn token(kind: TokenKind, span: Span, spacing: Spacing) -> Self {
    return Self::Token(Token::new(kind, span), spacing);
  }

  pub fn token_whitespaced(kind: TokenKind, span: Span) -> Self {
    return Self::token(kind, span, Spacing::Whitespaced);
  }
  pub fn token_sticked(kind: TokenKind, span: Span) -> Self {
    return Self::token(kind, span, Spacing::Sticked);
  }


  pub fn span(&self) -> Span {
    return match self {
      Self::Token(token, _) => token.span,

      Self::Delimited(delim_span, ..) => delim_span.entire(),
    };
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Spacing {
  Whitespaced,
  Sticked,
}
#[derive(Clone, Copy, Debug)]
pub struct DelimSpan {
  pub start: Span,
  pub end: Span,
}
impl DelimSpan {
  pub fn from_single(span: Span) -> Self {
    return Self {
      start: span,
      end: span,
    };
  }
  pub fn from_pair(start: Span, end: Span) -> Self {
    return Self { start, end };
  }

  pub fn entire(&self) -> Span {
    return self.start.with_hi(self.end.hi());
  }
}
#[derive(Clone, Copy, Debug)]
pub struct DelimSpacing {
  pub start: Spacing,
  pub end: Spacing,
}
impl DelimSpacing {
  pub fn new(start: Spacing, end: Spacing) -> Self {
    return Self { start, end };
  }
}


#[derive(Clone, Debug)]
pub struct TokenStream(pub Rc<Vec<TokenTree>>);
impl TokenStream {
  pub fn new(t_tree: Vec<TokenTree>) -> Self {
    return Self(Rc::new(t_tree));
  }


  pub fn into_tree(self) -> TokenTreeCursor {
    return TokenTreeCursor::new(self);
  }
}
