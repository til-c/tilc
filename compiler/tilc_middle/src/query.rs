#![allow(dead_code, unused_variables)]
#[macro_use]
mod query_system;

use query_system::*;

use tilc_macro::query;


query! {
  /// Test doc comment
  query resolver_for_lowering(_: ()) -> ::std::sync::Arc<::tilc_ast::Sandyq> {
    desc { "returns resolver and sandyq" }
  }
}
all_queries! { define_callbacks! }


pub(crate) fn default_query(name: &str, key: &dyn std::fmt::Debug) -> ! {
  panic!("tcx.{}({:?}) is not defined", name, key);
}
