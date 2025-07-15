mod query;
mod ty;

use std::ops::{Deref, DerefMut};

pub use query::*;
pub use ty::*;


#[derive(Debug, Default, Clone, Copy)]
pub struct Providers {
  pub queries: crate::query::Providers,
}
impl Deref for Providers {
  type Target = crate::query::Providers;

  fn deref(&self) -> &Self::Target {
    return &self.queries;
  }
}
impl DerefMut for Providers {
  fn deref_mut(&mut self) -> &mut Self::Target {
    return &mut self.queries;
  }
}
