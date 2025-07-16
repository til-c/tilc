#![allow(dead_code, unused_variables)]
#[macro_use]
mod query_system;
mod keys;

pub use keys::*;
pub use query_system::*;

use tilc_macro::query;


query! {
  query resolver_for_lowering(_: ()) -> ::std::sync::Arc<::tilc_ast::Sandyq> {
    desc { "returns resolver and sandyq" }
  }

  query crate_for_resolving((): ()) -> &'ctxt ::tilc_ast::Sandyq {
    feedable
    desc { "Sandyq before macro and name resolution" }
  }
}
all_queries! { define_callbacks! }
feedable_queries! { define_queries! }


pub(crate) fn default_query(name: &str, key: &dyn std::fmt::Debug) -> ! {
  panic!("tcx.{}({:?}) is not defined", name, key);
}
