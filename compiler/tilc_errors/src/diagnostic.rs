use std::{
  cell::RefCell,
  fmt::{Debug, Display},
  marker::PhantomData,
  ops::{Deref, DerefMut},
};

use tilc_span::{ErrorGuaranteed, Span};


pub type PResult<'a, T> = Result<T, Diag<'a>>;


pub trait Diagnostic<'a, E: EmissionGuarantee = ErrorGuaranteed> {
  fn into_diag(self, dcx: DiagCtxtHandle<'a>, level: Level) -> Diag<'a>;
}

#[derive(Clone, Copy, Debug)]
pub struct DiagCtxtInner {
  // TODO: make this struct useful

  // diag_flags:
}
impl DiagCtxtInner {
  pub fn new() -> Self {
    return Self {};
  }

  pub fn emit_guarantee(self) -> Option<ErrorGuaranteed> {
    todo!()
  }

  pub fn emit_diagnostic(
    &mut self,
    diagnostic: DiagInner,
  ) -> Option<ErrorGuaranteed> {
    match diagnostic.level {
      Level::Warning => return None,

      _ => {}
    };

    let is_error: bool = diagnostic.is_error();
    println!("{:#?}", self);
    println!("{:#?}", diagnostic);


    if is_error {
      return Some(ErrorGuaranteed::new_unchecked());
    } else {
      return None;
    };
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


  pub fn handle(&self) -> DiagCtxtHandle {
    return DiagCtxtHandle::new(self);
  }
}

#[derive(Clone, Copy, Debug)]
pub struct DiagCtxtHandle<'a> {
  dcx: &'a DiagCtxt,
}
impl<'a> Deref for DiagCtxtHandle<'a> {
  type Target = DiagCtxt;

  fn deref(&self) -> &Self::Target {
    return &self.dcx;
  }
}
impl<'a> DiagCtxtHandle<'a> {
  pub fn new(dcx: &'a DiagCtxt) -> Self {
    return Self { dcx };
  }

  pub fn emit_diagnostic(
    &self,
    diagnostic: DiagInner,
  ) -> Option<ErrorGuaranteed> {
    return self.inner.borrow_mut().emit_diagnostic(diagnostic);
  }

  pub fn struct_warning(self, message: String) -> Diag<'a> {
    return Diag::new(self, Level::Warning, message);
  }
  pub fn create_warning(self, diagnostic: impl Diagnostic<'a>) -> Diag<'a> {
    return diagnostic.into_diag(self, Level::Warning);
  }
  pub fn emit_warning(self, message: String) {
    self.struct_warning(message).emit();
  }

  pub fn struct_error(self, message: String) -> Diag<'a> {
    return Diag::new(self, Level::Error, message);
  }
  pub fn create_error(self, diagnostic: impl Diagnostic<'a>) -> Diag<'a> {
    return diagnostic.into_diag(self, Level::Error);
  }
  pub fn emit_error(self, diagnostic: impl Diagnostic<'a>) -> ErrorGuaranteed {
    return self.create_error(diagnostic).emit();
  }

  pub fn struct_falal(self, message: String) -> Diag<'a, FatalAbort> {
    return Diag::new(self, Level::Fatal, message);
  }
  pub fn emit_fatal(
    self,
    message: String,
  ) -> <FatalAbort as EmissionGuarantee>::EmissionResult {
    self.struct_falal(message).emit();

    // NOTE: If I switch to nightly redo fatal raising for diagnostics
    unreachable!();
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


  pub fn is_error(&self) -> bool {
    return match self.level {
      Level::Error | Level::Fatal => true,

      _ => false,
    };
  }

  pub fn span(&mut self, span: Span) {
    self.span = span;
  }
}
#[derive(Debug)]
pub struct Diag<'a, E: EmissionGuarantee = ErrorGuaranteed> {
  pub dcx: DiagCtxtHandle<'a>,
  diag: Option<Box<DiagInner>>,

  marker: PhantomData<E>,
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
impl<'a, E: EmissionGuarantee> Diag<'a, E> {
  pub fn new(dcx: DiagCtxtHandle<'a>, level: Level, message: String) -> Self {
    return Self {
      dcx,
      diag: Some(Box::new(DiagInner::new(level, message))),
      marker: PhantomData,
    };
  }
}
impl<'a, E: EmissionGuarantee> Diag<'a, E> {
  pub fn take_diag(&mut self) -> DiagInner {
    return *self.diag.take().unwrap();
  }
  pub fn emit(self) -> E::EmissionResult {
    return E::emit_guarantee(self);
  }

  pub fn emit_error_guaranteed(mut self) -> ErrorGuaranteed {
    let diag: DiagInner = self.take_diag();

    return self.dcx.emit_diagnostic(diag).unwrap();
  }
  pub fn emit_nothing(mut self) {
    let diag: DiagInner = self.take_diag();
    self.dcx.emit_diagnostic(diag);
  }
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


#[derive(Debug)]
pub struct FatalAbort;
impl EmissionGuarantee for FatalAbort {
  type EmissionResult = (); // !

  fn emit_guarantee(diag: Diag<'_, Self>) -> Self::EmissionResult {
    crate::FatalError.raise();
  }
}


// pub struct WarningGuaranteed;
// impl EmissionGuarantee for WarningGuaranteed {
//   type EmissionResult = ();

//   fn emit_guarantee(diag: Diag<'_, Self>) -> Self::EmissionResult {
//     return diag.emit_nothing();
//   }
// }

impl EmissionGuarantee for () {
  type EmissionResult = ();

  fn emit_guarantee(diag: Diag<'_, Self>) -> Self::EmissionResult {
    diag.emit();
  }
}
