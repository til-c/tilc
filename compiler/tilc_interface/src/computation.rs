use std::cell::{RefCell, RefMut};
use std::ops::Deref;
use std::sync::OnceLock;

use tilc_ast::Sandyq;

use crate::compiler::Compiler;
use crate::interface::Result;

#[derive(Debug)]
pub struct ComputationResult<'a, T>(RefMut<'a, T>);
impl<'a, T> Deref for ComputationResult<'a, T> {
  type Target = RefMut<'a, T>;

  fn deref(&self) -> &Self::Target {
    return &self.0;
  }
}

#[derive(Debug)]
pub struct ComputationTask<T>(RefCell<Option<Result<T>>>);
impl<T> Deref for ComputationTask<T> {
  type Target = RefCell<Option<Result<T>>>;

  fn deref(&self) -> &Self::Target {
    return &self.0;
  }
}
impl<T> ComputationTask<T> {
  pub fn compute<F: FnOnce() -> Result<T>>(
    &self,
    f: F,
  ) -> Result<ComputationResult<'_, T>> {
    return RefMut::filter_map(self.borrow_mut(), |r| {
      r.get_or_insert_with(|| f()).as_mut().ok()
    })
    .map_err(|r| *r.as_ref().unwrap().as_ref().map(|_| ()).unwrap_err())
    .map(ComputationResult);
  }
}

pub struct Computation<'cpl> {
  compiler: &'cpl Compiler,
  gcx: OnceLock<bool>,

  pub(crate) sandyq: ComputationTask<Sandyq>,
}
impl<'cpl> Computation<'cpl> {
  pub fn new(compiler: &'cpl Compiler) -> Self {
    return Self {
      compiler,
      gcx: OnceLock::new(),
      sandyq: ComputationTask(RefCell::new(None)),
    };
  }
  pub fn parse(&self) -> Result<ComputationResult<'_, Sandyq>> {
    return self.sandyq.compute(|| crate::parse(&self.compiler.session));
  }
}
