mod cache;
#[macro_use]
mod system;

pub use cache::*;
pub use system::*;

pub use queries::*;

use std::fmt::Debug;

use tilc_macros::queries;

queries! {
  query sandyq_for_resolver(_: ()) -> &'ctxt tilc_data_structures::Holder<tilc_ast::Sandyq> {
    feedable
  }

  query resolver_for_lowering_raw(_: ()) -> &'ctxt (crate::ResolverAstLowering, tilc_data_structures::Holder<std::sync::Arc<tilc_ast::Sandyq>>) {}
}

all_queries! { define_callbacks! }
feedable_queries! { define_feedables! }

pub(crate) fn default_query(name: &str, key: &dyn Debug) -> ! {
  panic!("tcx.{}({:?}) is not defined", name, key);
}
