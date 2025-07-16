#![allow(dead_code, unused_variables)]
#[macro_use]
mod query_system;
mod keys;

pub use keys::*;
pub use query_system::*;

use tilc_ast::Sandyq;
use tilc_data_structure::Holder;
use tilc_macro::tilc_queries;
use tilc_query_system::{QueryCache, try_get_cache};


tilc_queries! {
  query resolver_for_lowering(_: ()) -> &'ctxt Holder<Sandyq> {
    desc { "returns resolver and sandyq" }
  }

  query crate_for_resolving((): ()) -> &'ctxt Holder<Sandyq> {
    feedable
    desc { "Sandyq before macro and name resolution" }
  }
}
all_queries! { define_callbacks! }
feedable_queries! { define_queries! }


pub(crate) fn default_query(name: &str, key: &dyn std::fmt::Debug) -> ! {
  panic!("tcx.{}({:?}) is not defined", name, key);
}
