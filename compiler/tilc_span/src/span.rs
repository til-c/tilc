use tilc_macro::uidx;

use crate::{BytePos, LocalDefIdx, Pos};


uidx! {
  #[derive(Debug, PartialEq)]
  pub struct SpanContext {}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
  lo_or_index: u32,
  len: u16,
  ctxt_or_parent: u16,
}
impl Span {
  pub const EMPTY: Self = Self {
    lo_or_index: 0,
    len: 0,
    ctxt_or_parent: 0,
  };


  pub fn new(
    mut lo: BytePos,
    mut hi: BytePos,
    ctxt: SpanContext,
    parent: Option<LocalDefIdx>,
  ) -> Self {
    if lo > hi {
      std::mem::swap(&mut lo, &mut hi);
    };
    debug_assert!(lo < hi);
    let len = (hi - lo).0 as u16;

    match parent {
      Some(_) => todo!(),
      None => return TSpan::span(lo.to_u32(), len, ctxt.as_u16()),
    };
  }

  pub fn data(self) -> SpanData {
    let hi = self.lo_or_index.saturating_add(self.len as u32);
    return SpanData {
      lo: BytePos::from_u32(self.lo_or_index),
      hi: BytePos::from_u32(hi),
      ctxt: SpanContext::from_u16(self.ctxt_or_parent),
      parent: None,
    };
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


    return Self::new(self_data.lo, end_data.hi, self_data.ctxt, parent);
  }
  pub fn with_lo(self, lo: BytePos) -> Self {
    return self.data().with_lo(lo);
  }
  pub fn with_hi(self, hi: BytePos) -> Self {
    return self.data().with_hi(hi);
  }
}
pub struct TSpan {
  lo: u32,
  len: u16,
  ctxt: u16,
}
impl TSpan {
  pub const fn span(lo: u32, len: u16, ctxt: u16) -> Span {
    return Span {
      lo_or_index: lo,
      len,
      ctxt_or_parent: ctxt,
    };
  }
}


#[derive(Debug)]
pub struct SpanData {
  lo: BytePos,
  hi: BytePos,
  ctxt: SpanContext,
  parent: Option<LocalDefIdx>,
}
impl SpanData {
  pub fn span(self) -> Span {
    return Span::new(self.lo, self.hi, self.ctxt, self.parent);
  }

  pub fn with_lo(self, lo: BytePos) -> Span {
    return Span::new(lo, self.hi, self.ctxt, self.parent);
  }
  pub fn with_hi(self, hi: BytePos) -> Span {
    return Span::new(self.lo, hi, self.ctxt, self.parent);
  }
}
