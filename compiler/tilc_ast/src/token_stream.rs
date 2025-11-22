use std::rc::Rc;

use tilc_span::Span;

use crate::{Delim, Token, TokenKind};

#[derive(Debug)]
#[derive(Clone)]
pub struct TokenCursor {
  ttc: TokenTreeCursor,

  stack: Vec<(TokenTreeCursor, DelimSpan, DelimSpacing, Delim)>,
}
impl TokenCursor {
  pub const fn new(
    ttc: TokenTreeCursor,
    stack: Vec<(TokenTreeCursor, DelimSpan, DelimSpacing, Delim)>,
  ) -> Self {
    Self { ttc, stack }
  }

  pub fn step(&mut self) -> (Token, Spacing) {
    loop {
      if let Some(tt) = self.ttc.step() {
        match tt {
          &TokenTree::Token(token, spacing) => {
            return (token, spacing);
          }

          &TokenTree::Delimited(span, spacing, delim, ref t_stream) => {
            let ttc: TokenTreeCursor = t_stream.clone().into_tree();
            self.stack.push((self.ttc.clone(), span, spacing, delim));
            self.ttc = ttc;

            if delim != Delim::Empty {
              return (
                Token {
                  kind: TokenKind::OpenDelim(delim),
                  span: span.start,
                },
                spacing.start,
              );
            }
          }
        };
      } else if let Some((ttc, span, spacing, delim)) = self.stack.pop() {
        self.ttc = ttc;

        if delim != Delim::Empty {
          return (
            Token {
              kind: TokenKind::CloseDelim(delim),
              span: span.end,
            },
            spacing.end,
          );
        };
      } else {
        return (
          Token {
            kind: TokenKind::Eof,
            span: Span::EMPTY,
          },
          Spacing::Sticked,
        );
      };
    }
  }
}
#[derive(Debug)]
#[derive(Clone)]
pub struct TokenTreeCursor {
  stream: TokenStream,
  idx: usize,
}
impl TokenTreeCursor {
  const fn new(stream: TokenStream) -> Self {
    Self { stream, idx: 0 }
  }

  fn step(&mut self) -> Option<&TokenTree> {
    self.stream.0.get(self.idx).map(|token_tree: &TokenTree| {
      self.idx += 1;
      token_tree
    })
  }
}

#[derive(Debug)]
#[derive(Clone)]
pub struct TokenStream(pub Rc<Vec<TokenTree>>);
impl TokenStream {
  pub fn new(t_tree: Vec<TokenTree>) -> Self {
    Self(Rc::new(t_tree))
  }

  pub const fn into_tree(self) -> TokenTreeCursor {
    TokenTreeCursor::new(self)
  }
}

#[derive(Debug)]
#[derive(Clone)]
pub enum TokenTree {
  /// Single token and spacing info
  Token(Token, Spacing),

  /// Sequence of tokens
  Delimited(DelimSpan, DelimSpacing, Delim, TokenStream),
}

#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum Spacing {
  Whitespaced,
  Sticked,
}
#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct DelimSpan {
  pub start: Span,
  pub end: Span,
}
#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct DelimSpacing {
  pub start: Spacing,
  pub end: Spacing,
}
