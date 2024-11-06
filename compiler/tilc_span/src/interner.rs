use std::sync::{RwLock, RwLockWriteGuard};

use indexmap::IndexSet;

use crate::Symbol;

pub struct Interner(RwLock<InnerInterner>);
impl Interner {
  pub fn new() -> Self {
    return Self(RwLock::new(InnerInterner {
      strings: IndexSet::new(),
    }));
  }

  pub fn intern(&self, string: &str) -> Symbol {
    let mut inner: RwLockWriteGuard<'_, InnerInterner> = self.lock();
    if let Some(idx) = inner.strings.get_index_of(string) {
      return Symbol::new(idx as u32);
    }


    // let string: &'static str = unsafe { &*(string as *const str) };
    let string: &'static str = string.to_string().leak();
    let (idx, is_new): (usize, bool) = inner.strings.insert_full(string);


    debug_assert!(is_new);
    return Symbol::new(idx as u32);
  }

  pub fn get(&self, symbol: Symbol) -> &str {
    return self.lock().strings.get_index(symbol.idx()).unwrap();
  }

  fn lock(&self) -> RwLockWriteGuard<'_, InnerInterner> {
    return self.0.write().unwrap();
  }
}

pub struct InnerInterner {
  strings: IndexSet<&'static str>,
}
