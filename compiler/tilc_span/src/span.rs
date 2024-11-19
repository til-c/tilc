use crate::Pos;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Span {
  pub start: u32,
  pub len: u16,

  pub ctxt: u16,
}
impl Span {
  pub const EMPTY: Self = Self {
    start: u32::MAX,
    len: 0,
    ctxt: u16::MAX,
  };

  pub fn new(start: Pos, end: Pos, ctxt: u16) -> Self {
    debug_assert!(start <= end);


    let len: u16 = (end - start).into();
    return Self {
      start: start.into(),
      len,
      ctxt,
    };
  }
  pub fn from_u32(start: u32, end: u32, ctxt: u16) -> Self {
    debug_assert!(start <= end);

    let len: u16 = (end - start) as u16;
    return Self { start, len, ctxt };
  }
  fn end(&self) -> u32 {
    return self.start + self.len as u32;
  }


  pub fn to(self, other: Self) -> Self {
    let len: u16 = (other.end() - self.start) as u16;
    return Self {
      start: self.start,
      len,
      ctxt: self.ctxt,
    };
  }
}
