mod arena;
mod ast;
mod context;
mod keys;
mod passes;
mod query;

pub use arena::*;
pub use ast::*;
pub use context::*;
pub use keys::*;
pub use query::*;

pub(crate) use passes::*;

use std::sync::LazyLock;

pub static DEFAULT_QUERY_PROVIDERS: LazyLock<Providers> = LazyLock::new(|| {
  let mut providers = Providers::new();

  providers.resolver_for_lowering_raw = resolver_for_lowering_raw;
  // providers.a = |tcx, k| k + 1;

  providers
});
