use std::sync::LazyLock;

use tilc_middle::Providers;
use tilc_span::ErrorGuaranteed;

use crate::resolver_for_lowering;


pub type Result<T> = std::result::Result<T, ErrorGuaranteed>;

pub static DEFAULT_QUERY_PROVIDERS: LazyLock<Providers> = LazyLock::new(|| {
  let providers = &mut Providers::default();

  providers.resolver_for_lowering = resolver_for_lowering;


  return *providers;
});
