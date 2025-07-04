use indexmap::IndexSet;
use parking_lot::RwLock;

use crate::Symbol;

pub struct Interner(RwLock<InnerInterner>);
impl Interner {
  pub fn new() -> Self {
    return Self(RwLock::new(InnerInterner {
      strings: IndexSet::new(),
    }));
  }
  pub(crate) fn prefill(init: &[&'static str]) -> Self {
    return Interner(RwLock::new(InnerInterner {
      strings: init.iter().copied().collect(),
    }));
  }

  pub(crate) fn intern(&self, string: &str) -> Symbol {
    let mut inner = self.0.write();
    if let Some(idx) = inner.strings.get_index_of(string) {
      return Symbol::new(idx as u32);
    };

    let string = string.to_string().leak();
    let (idx, is_new) = inner.strings.insert_full(string);

    debug_assert!(!is_new);
    return Symbol::new(idx as u32);
  }
  pub(crate) fn get(&self, symbol: Symbol) -> &str {
    let inner = self.0.write();
    return inner.strings.get_index(symbol.idx()).unwrap();
  }
}


pub struct InnerInterner {
  strings: IndexSet<&'static str>,
}
