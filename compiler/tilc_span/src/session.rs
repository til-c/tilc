use crate::Interner;

pub struct SessionGlobals {
  pub(crate) symbol_interner: Interner,
}
impl SessionGlobals {
  pub(crate) fn new() -> Self {
    return Self {
      symbol_interner: Interner::with_prefilled(),
    };
  }
}
