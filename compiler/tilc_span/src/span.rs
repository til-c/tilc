use tilc_macros::uidx;

use crate::{BytePos, LocalDefIdx, Pos};

uidx! {
  pub struct SpanCtxt {
    const ROOT = 0;
  }
}

#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub struct Span {
  lo_or_idx: u32,
  len: u16,
  ctxt_or_parent: u16,
}
impl Span {
  pub const EMPTY: Self = Self {
    lo_or_idx: 0,
    len: 0,
    ctxt_or_parent: 0,
  };

  pub fn new(
    mut lo: BytePos,
    mut hi: BytePos,
    ctxt: SpanCtxt,
    parent: Option<LocalDefIdx>,
  ) -> Self {
    if hi < lo {
      std::mem::swap(&mut lo, &mut hi);
    };
    debug_assert!(lo <= hi);
    let len = (hi - lo).0 as u16;

    match parent {
      Some(_) => todo!(),
      None => TSpan::span(lo.to_u32(), len, ctxt.as_u16()),
    }
  }

  fn data(self) -> SpanData {
    let hi = self.lo_or_idx.saturating_add(self.len as u32);
    SpanData {
      lo: BytePos::from_u32(self.lo_or_idx),
      hi: BytePos::from_u32(hi),
      ctxt: SpanCtxt::from_u16(self.ctxt_or_parent),
      parent: None,
    }
  }

  pub fn to(self, end: Self) -> Self {
    let self_data: SpanData = self.data();
    let end_data: SpanData = end.data();
    if self_data.ctxt != end_data.ctxt {
      todo!();
    };
    let parent: Option<LocalDefIdx> = if self_data.parent == end_data.parent {
      self_data.parent
    } else {
      None
    };

    Self::new(self_data.lo, end_data.hi, self_data.ctxt, parent)
  }
  fn with_lo(self, lo: BytePos) -> Self {
    self.data().with_lo(lo)
  }
  fn with_hi(self, hi: BytePos) -> Self {
    self.data().with_hi(hi)
  }

  pub fn shrink_to_lo(self) -> Self {
    let data = self.data();
    data.with_hi(data.lo)
  }
  pub fn shrink_to_hi(self) -> Self {
    let data: SpanData = self.data();
    data.with_lo(data.hi)
  }
}

struct TSpan {}
impl TSpan {
  const fn span(lo: u32, len: u16, ctxt: u16) -> Span {
    Span {
      lo_or_idx: lo,
      len,
      ctxt_or_parent: ctxt,
    }
  }
}

#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct SpanData {
  lo: BytePos,
  hi: BytePos,
  ctxt: SpanCtxt,
  parent: Option<LocalDefIdx>,
}
impl SpanData {
  pub fn span(self) -> Span {
    Span::new(self.lo, self.hi, self.ctxt, self.parent)
  }

  pub fn with_lo(self, lo: BytePos) -> Span {
    Span::new(lo, self.hi, self.ctxt, self.parent)
  }
  pub fn with_hi(self, hi: BytePos) -> Span {
    Span::new(self.lo, hi, self.ctxt, self.parent)
  }
}
