use tilc_index::uidx;

use crate::{BytePos, LocalDefIdx};


uidx! {
  #[derive(Clone, Copy)]
  #[derive(PartialEq)]
  pub struct SpanContext;
}


impl SpanContext {
  pub const ROOT: Self = Self(0);
}

#[derive(Clone, Copy, Debug, PartialEq)]
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
    if hi < lo {
      std::mem::swap(&mut lo, &mut hi);
    };
    debug_assert!(lo <= hi);


    let (len, ctxt32) = ((hi.0 - lo.0), ctxt.0);
    match parent {
      Some(_) => todo!(),
      None => return TSpan::span(lo.0, len as u16, ctxt32 as u16),
    };
  }
  pub fn data(self) -> SpanData {
    let hi: u32 = self.lo_or_index.saturating_add(self.len as u32);
    return SpanData {
      lo: BytePos(self.lo_or_index),
      hi: BytePos(hi),
      ctxt: SpanContext(self.ctxt_or_parent as u32),
      parent: None,
    };
  }
  pub fn lo(self) -> BytePos {
    return self.data().lo;
  }
  pub fn hi(self) -> BytePos {
    return self.data().hi;
  }


  pub fn with_lo(self, lo: BytePos) -> Self {
    return self.data().with_lo(lo);
  }
  pub fn with_hi(self, hi: BytePos) -> Self {
    return self.data().with_hi(hi);
  }


  pub fn shrink_to_lo(self) -> Self {
    let data: SpanData = self.data();
    return data.with_hi(data.lo);
  }
  pub fn shrink_to_hi(self) -> Self {
    let data: SpanData = self.data();
    return data.with_lo(data.hi);
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
pub struct InternedSpan {
  index: u32,
}


pub struct SpanData {
  lo: BytePos,
  hi: BytePos,
  ctxt: SpanContext,
  parent: Option<LocalDefIdx>,
}
impl SpanData {
  pub fn span(&self) -> Span {
    return Span::new(self.lo, self.hi, self.ctxt, self.parent);
  }
  pub fn with_lo(&self, lo: BytePos) -> Span {
    return Span::new(lo, self.hi, self.ctxt, self.parent);
  }
  pub fn with_hi(&self, hi: BytePos) -> Span {
    return Span::new(self.lo, hi, self.ctxt, self.parent);
  }


  pub fn is_empty(&self) -> bool {
    return self.lo.0 == 0 && self.hi.0 == 0;
  }
}
