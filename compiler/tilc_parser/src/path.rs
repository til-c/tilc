use tilc_ast::{Delim, NodeIdx, Path, PathSegment, Spacing, TokenKind};
use tilc_errors::PResult;
use tilc_span::{Identifier, Span};

use crate::Parser;

impl<'a> Parser<'a> {
  pub(crate) fn parse_path(&mut self, is_mod: bool) -> PResult<'a, Box<Path>> {
    let span: Span = self.token.span;
    let mut segments: Vec<PathSegment> = Vec::new();


    loop {
      let segment: PathSegment = self.parse_path_segment()?;
      segments.push(segment);

      self.parse_path_seq();


      if self.check(TokenKind::Semicolon)
        || (!is_mod && self.token_spacing == Spacing::Whitespaced)
      {
        println!("{:#?}", self.token);
        println!("{:#?}", self.token_spacing);
        println!("{:#?}", segments);

        span.to(self.prev_token.span);
        self.step();
        break;
      };
    }

    return Ok(Box::new(Path { segments, span }));
  }

  pub(crate) fn parse_path_segment(&mut self) -> PResult<'a, PathSegment> {
    if self.step_if(TokenKind::OpenDelim(Delim::Brace)) {
      todo!("{:#?}", self.token);
    };
    let ident: Identifier = self.parse_ident()?;

    return Ok(PathSegment {
      ident,
      idx: NodeIdx::EMPTY,
    });
  }

  pub(crate) fn parse_path_seq(&mut self) -> bool {
    return self.step_if(TokenKind::Path);
  }
}
