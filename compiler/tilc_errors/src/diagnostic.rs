use std::path::PathBuf;


use tilc_span::ErrorGuaranteed;


pub type PResult<'a, T> = Result<T, Diag<'a>>;


pub struct DiagCtxt {}
impl DiagCtxt {}


pub enum Level {
  /// Skill issues
  Warning,

  /// Failure during compilation stage
  Error,

  /// Failure during preparation to compilation stage
  Fatal,
}
pub struct DiagLocation {
  pub file: PathBuf,
  pub line: u32,
  pub col: u32,
}
pub struct DiagInner {
  pub level: Level,

  pub location: DiagLocation,
}
pub struct Diag<'a, E: EmissionGuarantee = ErrorGuaranteed> {
  pub dcx: &'a DiagCtxt,

  diag: Option<DiagInner>,

  _marker: std::marker::PhantomData<E>,
}
impl Diag<'_> {
  pub fn emit(self) -> ErrorGuaranteed {
    todo!()
  }
}

pub trait EmissionGuarantee {}
impl EmissionGuarantee for ErrorGuaranteed {}
