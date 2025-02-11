use tilc_index::uidx;

use crate::SandyqId;


uidx! {
  #[derive(Clone, Copy)]
  #[derive(Debug, PartialEq)]
  pub struct LocalDefIdx;
}


pub struct DefIdx {
  pub sandyq: SandyqId,
  pub local_def_id: LocalDefIdx,
}
