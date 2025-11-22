mod passes;
mod util;

pub use passes::*;
pub use util::*;

use tilc_error::ErrorGuaranteed;

pub type Result<T> = ::std::result::Result<T, ErrorGuaranteed>;
