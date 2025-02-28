use tilc_ast::{
  BinOp, Delim, NodeIdx, Path, PathSegment, TokenKind, Use, UseKind,
};
use tilc_errors::PResult;
use tilc_span::{kw, Identifier, Span};

use crate::Parser;


pub(crate) enum PathStyle {
  Mod,
  Type,
}
#[derive(Debug)]
enum PathSeqType {
  Default,
  Multiple,
  Everything,
}


impl<'a> Parser<'a> {
  pub(crate) fn parse_use_path(&mut self) -> PResult<'a, Use> {
    // ash std::mem;
    //     ^
    //     | token's position
    let lo: Span = self.token.span;
    let mut prefix = Path {
      segments: Vec::new(),
      span: lo,
    };

    let mut kind: UseKind = UseKind::Single(None);
    if self.check_use_bundler() {
      todo!();
    } else {
      prefix = self.parse_path()?;
    };

    // ash std::mem;
    //             ^
    //             | token's position
    if !self.check(TokenKind::Semicolon) {
      if self.check_use_bundler() {
        // ash std::mem::*;
        //             ^
        //             | token's position
        self.parse_path_seq();
        if self.step_if(TokenKind::OpenDelim(Delim::Brace)) {
          // ash std::mem::{replace, swap};
          //               ^
          //               | token's position
          todo!("parse inner use");
        } else if self.step_if(TokenKind::BinOp(BinOp::Star)) {
          // ash std::mem::*;
          //               ^
          //               | token's position
          kind = UseKind::Everything;
        };
      } else {
        // ash std::mem m sekildi;
        //              ^
        //              | token's position
        kind = UseKind::Single(Some(self.parse_ident()?));
        self.expect_kw(kw::As)?;
      };
    };
    dbg!(&kind);
    dbg!(&prefix);

    return Ok(Use {
      prefix: Box::new(prefix),
      kind,
      span: lo.to(self.prev_token.span),
    });
  }
  pub(crate) fn parse_path(&mut self) -> PResult<'a, Path> {
    // use std::mem;
    //     ^
    //     | span's position
    let lo: Span = self.token.span;
    let mut segments: Vec<PathSegment> = Vec::new();
    loop {
      let segment: PathSegment = self.parse_path_segment()?;
      segments.push(segment);

      if self.check_use_bundler() || !self.parse_path_seq() {
        // ash std::mem;
        //             ^
        //              \
        // ash std::mem::{replace, swap};
        //             ^
        //             |
        // ash std::mem::*;
        //             ^
        //             | self.token
        //               must be ';' or "::{" or "::*"
        break;
      };
    }

    return Ok(Path {
      segments,
      span: lo.to(self.prev_token.span),
    });
  }


  fn parse_path_segment(&mut self) -> PResult<'a, PathSegment> {
    let ident: Identifier = self.parse_path_segment_ident()?;

    return Ok(PathSegment {
      ident,
      idx: NodeIdx::EMPTY,
    });
  }
  fn parse_path_segment_ident(&mut self) -> PResult<'a, Identifier> {
    return match self.token.ident() {
      Some((ident, false)) if ident.is_path_segment_ident() => Ok(ident),
      _ => self.parse_ident(),
    };
  }

  fn parse_path_seq(&mut self) -> bool {
    if self.step_if(TokenKind::Path) {
      return true;
    } else if self.check(TokenKind::Colon) {
      todo!("use double colon");
    } else if self.check(TokenKind::Dot) {
      todo!("use double colon instead of dot")
    } else {
      return false;
    };
  }

  fn check_use_bundler(&self) -> bool {
    if self.check(TokenKind::Path) {
      return self.look_ahead_and(1, |t| {
        matches!(
          t.kind,
          TokenKind::OpenDelim(Delim::Brace) | TokenKind::BinOp(BinOp::Star)
        )
      });
    } else {
      return false;
    };
  }
}
