use tilc_index::uidx;
use tilc_span::LocalDefIdx;

uidx! {
  #[derive(Debug)]
  pub struct ItemIdx;
}

#[derive(Debug)]
pub struct HirId {
  pub owner: OwnerId,
  pub local_idx: ItemIdx,
}
#[derive(Debug)]
pub struct OwnerId {
  pub def_id: LocalDefIdx,
}
