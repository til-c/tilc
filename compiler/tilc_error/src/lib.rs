mod diagnostic;

pub use diagnostic::*;

use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use tilc_span::Span;

pub type PResult<'a, T> = ::core::result::Result<T, Diag<'a>>;

#[derive(Debug)]
pub struct Diag<'a, E = ErrorGuaranteed>
where
  E: EmissionGuarantee, {
  pub dcx: DiagCtxtHandle<'a>,
  diag: Option<Box<DiagInner>>,
  marker: PhantomData<E>,
}
impl<'a, E> Diag<'a, E>
where
  E: EmissionGuarantee,
{
  pub fn emit(self) -> E::EmissionResult {
    E::emit_guarantee(self)
  }
}
#[derive(Debug)]
pub struct DiagInner {
  pub level: Level,
  pub message: Rc<str>,

  pub span: Span,
}
#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct DiagCtxtHandle<'a> {
  dcx: &'a DiagCtxt,
}
#[derive(Debug)]
pub struct DiagCtxt {
  inner: RefCell<DiagCtxtInner>,
}
#[derive(Debug)]
struct DiagCtxtInner {}

#[derive(Debug)]
pub enum Level {
  /// Just some additional info
  Note,

  /// Skill issues
  Warning,

  /// Failure during compilation stage
  Error,

  /// Failure during preparation to compilation stage
  Fatal,
}

pub struct FatalError;
impl FatalError {
  pub fn raise(self) -> ! {
    std::panic::resume_unwind(Box::new(FatalError));
  }
}

#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct ErrorGuaranteed(());
impl ErrorGuaranteed {
  const unsafe fn new_unchecked() -> Self {
    Self(())
  }
}
impl EmissionGuarantee for ErrorGuaranteed {
  type EmissionResult = Self;
  fn emit_guarantee(diag: Diag<'_, Self>) -> Self::EmissionResult {
    todo!();
    // diag.emit_error_guaranteed()
  }
}

pub trait EmissionGuarantee: Sized {
  type EmissionResult;

  fn emit_guarantee(diag: Diag<'_, Self>) -> Self::EmissionResult;
}
