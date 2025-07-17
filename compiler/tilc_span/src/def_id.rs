use std::hash::{DefaultHasher, Hash, Hasher};

use tilc_data_structure::Hash64;
use tilc_macro::uidx;

use crate::Symbol;


uidx! {
  #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
  pub struct DefIdx {}
}
uidx! {
  #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
  pub struct SandyqIdx {}
}
pub const LOCAL_SANDYQ: SandyqIdx = SandyqIdx::EMPTY;


#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LocalDefIdx {
  local: DefIdx,
}
impl LocalDefIdx {
  pub fn to_def_id(self) -> DefId {
    return DefId {
      def_idx: self.local,
      sandyq_idx: LOCAL_SANDYQ,
    };
  }
}

pub const SANDYQ_DEF_IDX: LocalDefIdx = LocalDefIdx {
  local: DefIdx::EMPTY,
};

#[derive(Debug, Hash)]
pub struct SandyqId(Hash64);
impl SandyqId {
  pub fn new(crate_name: Symbol, is_exe: bool) -> Self {
    let mut hasher = DefaultHasher::new();

    crate_name.hash(&mut hasher);
    hasher.write(if is_exe { b"exe" } else { b"lib" });


    return Self(Hash64::new(hasher.finish()));
  }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct DefId {
  pub def_idx: DefIdx,
  pub sandyq_idx: SandyqIdx,
}
