#[derive(Debug, PartialEq)]
pub struct Span {
  pub start: u32,
  pub end: u32,
}
impl Span {
  pub const EMPTY: Self = Self::new(u32::MAX, u32::MAX);

  pub const fn new(start: u32, end: u32) -> Self {
    debug_assert!(start <= end);


    return Self { start, end };
  }
}
