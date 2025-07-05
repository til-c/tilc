use std::rc::Rc;

use tilc_span::Span;

use crate::{Delim, Token, TokenKind};


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Spacing {
  Whitespaced,
  Sticked,
}

#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct TokenTreeCursor {
  stream: TokenStream,
  idx: usize,
}
impl TokenTreeCursor {
  pub fn new(stream: TokenStream) -> Self {
    return Self { stream, idx: 0 };
  }

  fn step(&mut self) -> Option<&TokenTree> {
    return self.stream.0.get(self.idx).map(|token_tree: &TokenTree| {
      self.idx += 1;
      token_tree
    });
  }
}

#[derive(Debug, Clone)]
pub struct TokenStream(pub Rc<Vec<TokenTree>>);
impl TokenStream {
  pub fn new(t_tree: Vec<TokenTree>) -> Self {
    return Self(Rc::new(t_tree));
  }


  pub fn into_tree(self) -> TokenTreeCursor {
    return TokenTreeCursor::new(self);
  }
}

#[derive(Debug, Clone)]
pub enum TokenTree {
  /// Single token and spacing info
  Token(Token, Spacing),

  /// Sequence of tokens
  Delimited(DelimSpan, DelimSpacing, Delim, TokenStream),
}


#[derive(Debug, Clone, Copy)]
pub struct DelimSpan {
  pub start: Span,
  pub end: Span,
}
#[derive(Debug, Clone, Copy)]
pub struct DelimSpacing {
  pub start: Spacing,
  pub end: Spacing,
}
