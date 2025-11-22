use std::{
  collections::{HashMap, HashSet},
  hash::{BuildHasherDefault, Hasher},
};

pub type UnHashMap<K, V> = HashMap<K, V, BuildHasherDefault<Unhasher>>;
pub type UnHashSet<V> = HashSet<V, BuildHasherDefault<Unhasher>>;

#[derive(Default)]
pub struct Unhasher {
  pub value: u64,
}
impl Hasher for Unhasher {
  fn finish(&self) -> u64 {
    self.value
  }

  fn write(&mut self, _: &[u8]) {
    unimplemented!("use write_u64");
  }
  fn write_u64(&mut self, i: u64) {
    debug_assert_eq!(
      0, self.value,
      "Unhaser do not hash things, use `Hash64` instead"
    );

    self.value = i;
  }
}
