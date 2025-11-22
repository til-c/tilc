use std::hash::{DefaultHasher, Hash, Hasher};

use tilc_data_structures::Hash64;
use tilc_macros::uidx;

use crate::Symbol;

uidx! {
  pub struct DefIdx {}

}

#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
pub struct LocalDefIdx {
  local: DefIdx,
}

#[derive(Debug)]
#[derive(Hash)]
pub struct SandyqId(Hash64);
impl SandyqId {
  fn new(crate_name: Symbol, is_exe: bool) -> Self {
    let mut hasher = DefaultHasher::new();

    crate_name.hash(&mut hasher);
    hasher.write(if is_exe { b"exe" } else { b"lib" });

    Self(Hash64::new(hasher.finish()))
  }
}
