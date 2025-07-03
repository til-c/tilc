use std::hash::{DefaultHasher, Hash, Hasher};

use tilc_data_structure::Hash64;


#[derive(Debug, Hash)]
pub struct SandyqId(Hash64);
impl SandyqId {
  pub fn new(crate_name: String, is_exe: bool) -> Self {
    let mut hasher = DefaultHasher::new();

    crate_name.hash(&mut hasher);
    hasher.write(if is_exe { b"exe" } else { b"lib" });


    return Self(Hash64::new(hasher.finish()));
  }
}
