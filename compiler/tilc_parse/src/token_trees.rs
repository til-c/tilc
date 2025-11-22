use tilc_advanced_lexer::TokenReader;
use tilc_ast::{Delim, DelimSpacing, DelimSpan, Spacing, Token, TokenKind, TokenStream, TokenTree};
use tilc_error::ErrorGuaranteed;

pub(crate) struct TokenTreesReader<'psess, 'lex> {
  token_reader: TokenReader<'psess, 'lex>,
  token: Token,
}
impl<'psess, 'lex> TokenTreesReader<'psess, 'lex> {
  pub(crate) fn new(token_reader: TokenReader<'psess, 'lex>) -> Self {
    Self {
      token_reader,
      token: Token::DUMMY,
    }
  }

  pub(crate) fn lex_token_trees(
    &mut self,
    from_delim: bool,
  ) -> (Spacing, TokenStream, Result<(), ErrorGuaranteed>) {
    let (spacing, _) = self.step(false);

    let mut buffer = Vec::new();
    loop {
      match self.token.kind {
        // TODO: Check for from_delim, return err in case from_delim is true
        TokenKind::Eof => return (spacing, TokenStream::new(buffer), Ok(())),

        TokenKind::OpenDelim(delim) => {
          match self.lex_token_tree_in_open_delim(delim) {
            Ok(token_tree) => buffer.push(token_tree),
            Err(_) => todo!(),
          };
        }
        TokenKind::CloseDelim(_delim) => {
          // TODO: Implement Diagnostics
          // Note: Do not put close delim token inside buffer
          //       Close dedlims must be handled in `Self::lex_token_tree_in_open_delim`'
          // Maybe there is a better way but idk
          return (
            spacing,
            TokenStream::new(buffer),
            if from_delim { Ok(()) } else { todo!() },
          );
        }

        _ => {
          let (following_spacing, token) = self.step(true);
          buffer.push(TokenTree::Token(token, following_spacing));
        }
      }
    }
  }
  fn lex_token_tree_in_open_delim(
    &mut self,
    opening_delim: Delim,
  ) -> Result<TokenTree, ErrorGuaranteed> {
    let delim_start_span = self.token.span;
    let (spacing, token_stream, err) = self.lex_token_trees(true);
    if let Err(_) = err {
      todo!("Error handling is not implemented yet");
    };

    let delim_span = DelimSpan {
      start: delim_start_span,
      end: self.token.span,
    };
    let close_spacing = match self.token.kind {
      // Case when closing and opening delims match
      // If delims match just step one token forward
      TokenKind::CloseDelim(delim) if delim == opening_delim => self.step(true).0,

      // Case when delims do not match
      TokenKind::CloseDelim(delim) => {
        todo!();
      }
      TokenKind::Eof => Spacing::Whitespaced,

      _ => unreachable!("It was supposed to be unreachble part of the code, what did you do?"),
    };

    let delim_spacing = DelimSpacing {
      start: spacing,
      end: close_spacing,
    };
    Ok(TokenTree::Delimited(
      delim_span,
      delim_spacing,
      opening_delim,
      token_stream,
    ))
  }
  fn step(&mut self, glue: bool) -> (Spacing, Token) {
    let (spacing, next_token) = loop {
      let (next_token, is_whitespaced) = self.token_reader.next_token();

      if is_whitespaced {
        break (Spacing::Whitespaced, next_token);
      } else if glue && self.token.glueable(next_token).is_some() {
        self.token = self
          .token
          .glueable(next_token)
          .unwrap_or_else(|| unreachable!());
      } else {
        let spacing = if self.token.kind == TokenKind::Eof {
          Spacing::Whitespaced
        } else {
          Spacing::Sticked
        };

        break (spacing, next_token);
      }
    };

    let current_token = std::mem::replace(&mut self.token, next_token);
    (spacing, current_token)
  }
}
