pub enum Edition {
  // Don't know the release year yet
  E20xx,

  // I'll try to create features every 3-5 years
  E20xxPlus3or5,
}
impl Default for Edition {
  fn default() -> Self {
    return Self::E20xx;
  }
}
