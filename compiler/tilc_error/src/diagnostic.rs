use std::{
  cell::RefCell,
  marker::PhantomData,
  ops::{Deref, DerefMut},
};

use tilc_span::{ErrorGuaranteed, Span};

pub type PResult<'a, T> = Result<T, Diag<'a>>;


#[derive(Debug)]
pub struct Diag<'a, E: EmissionGuarantee = ErrorGuaranteed> {
  pub dcx: DiagCtxtHandle<'a>,
  diag: Option<Box<DiagInner>>,
  marker: PhantomData<E>,
}
impl<'a, E: EmissionGuarantee> Diag<'a, E> {
  pub fn new(dcx: DiagCtxtHandle<'a>, level: Level, message: String) -> Self {
    return Self {
      dcx,
      diag: Some(Box::new(DiagInner::new(level, message))),
      marker: PhantomData,
    };
  }
  pub fn emit(self) -> E::EmissionResult {
    return E::emit_guarantee(self);
  }

  fn take_diag(&mut self) -> DiagInner {
    return *self.diag.take().unwrap();
  }
  fn emit_error_guaranteed(mut self) -> ErrorGuaranteed {
    let diag = self.take_diag();

    return self.dcx.emit_diagnostic(diag).unwrap();
  }
}
impl<'a, E: EmissionGuarantee> Deref for Diag<'a, E> {
  type Target = DiagInner;

  fn deref(&self) -> &Self::Target {
    return self.diag.as_ref().unwrap();
  }
}
impl<'a, E: EmissionGuarantee> DerefMut for Diag<'a, E> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    return self.diag.as_mut().unwrap();
  }
}

#[derive(Clone, Copy, Debug)]
pub struct DiagCtxtHandle<'a> {
  dcx: &'a DiagCtxt,
}
impl<'a> DiagCtxtHandle<'a> {
  pub fn new(dcx: &'a DiagCtxt) -> Self {
    return Self { dcx };
  }

  fn emit_diagnostic(&self, diagnostic: DiagInner) -> Option<ErrorGuaranteed> {
    return self.inner.borrow_mut().emit_diagnostic(diagnostic);
  }
}
impl<'a> Deref for DiagCtxtHandle<'a> {
  type Target = DiagCtxt;

  fn deref(&self) -> &Self::Target {
    return &self.dcx;
  }
}

#[derive(Debug)]
pub struct DiagCtxt {
  inner: RefCell<DiagCtxtInner>,
}
impl DiagCtxt {
  pub fn new() -> Self {
    return Self {
      inner: RefCell::new(DiagCtxtInner::new()),
    };
  }


  fn handle(&self) -> DiagCtxtHandle {
    return DiagCtxtHandle::new(self);
  }
}
#[derive(Clone, Copy, Debug)]
pub struct DiagCtxtInner {
  // TODO: make this struct useful
}
impl DiagCtxtInner {
  pub fn new() -> Self {
    return Self {};
  }

  fn emit_diagnostic(
    &mut self,
    diagnostic: DiagInner,
  ) -> Option<ErrorGuaranteed> {
    match diagnostic.level {
      Level::Warning => return None,

      _ => {}
    };

    let is_error: bool = diagnostic.is_error();


    if is_error {
      return Some(ErrorGuaranteed::new_unchecked());
    } else {
      return None;
    };
  }
}

#[derive(Debug)]
pub struct DiagInner {
  pub level: Level,
  pub message: String,

  pub span: Span,
}
impl DiagInner {
  pub fn new(level: Level, message: String) -> Self {
    return Self {
      level,
      message,

      span: Span::EMPTY,
    };
  }
  fn is_error(&self) -> bool {
    return match self.level {
      Level::Error | Level::Fatal => true,

      _ => false,
    };
  }
}
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


pub trait EmissionGuarantee: Sized {
  type EmissionResult;

  fn emit_guarantee(diag: Diag<'_, Self>) -> Self::EmissionResult;
}
impl EmissionGuarantee for ErrorGuaranteed {
  type EmissionResult = Self;
  fn emit_guarantee(diag: Diag<'_, Self>) -> Self::EmissionResult {
    return diag.emit_error_guaranteed();
  }
}
