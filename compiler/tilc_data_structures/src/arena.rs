use std::{
  alloc::Layout,
  cell::{Cell, RefCell},
  cmp,
  mem::{forget, needs_drop},
  ptr::{NonNull, null_mut, write},
};

const PAGE: usize = 4096;
const HUGE_PAGE: usize = 2 * 1024 * 1024;

#[derive(Debug)]
pub struct DroplessArena {
  start: Cell<*mut u8>,
  end: Cell<*mut u8>,

  chunks: RefCell<Vec<ArenaChunk>>,
}
impl DroplessArena {
  pub const fn new() -> Self {
    Self {
      start: Cell::new(null_mut()),
      end: Cell::new(null_mut()),

      chunks: RefCell::new(Vec::new()),
    }
  }

  #[inline]
  pub fn alloc<T>(&self, object: T) -> &mut T {
    assert!(!needs_drop::<T>());

    let mem = self.alloc_raw(Layout::for_value::<T>(&object)) as *mut T;

    unsafe {
      write(mem, object);
      return &mut *mem;
    };
  }

  fn alloc_raw(&self, layout: Layout) -> *mut u8 {
    assert_ne!(layout.size(), 0);

    loop {
      if let Some(a) = self.alloc_raw_without_grow(layout) {
        break a;
      };

      self.grow(layout.size());
    }
  }
  #[inline]
  fn alloc_raw_without_grow(&self, layout: Layout) -> Option<*mut u8> {
    let start = self.start.get().addr();
    let old_end = self.end.get();
    let end = old_end.addr();

    let align = layout.align();
    let bytes = layout.size();

    let new_end = end.checked_sub(bytes)? & !(align - 1);
    if start <= new_end {
      let new_end = old_end.with_addr(new_end);
      self.end.set(new_end);
      return Some(new_end);
    } else {
      return None;
    };
  }

  #[inline(never)]
  fn grow(&self, additional: usize) {
    let mut chunks = self.chunks.borrow_mut();
    let mut new_cap;
    if let Some(last_chunk) = chunks.last_mut() {
      new_cap = last_chunk.capacity.min(HUGE_PAGE / 2);
      new_cap *= 2;
    } else {
      new_cap = PAGE;
    };
    new_cap = cmp::max(additional, new_cap);

    let mut chunk = unsafe { ArenaChunk::new(new_cap) };
    self.start.set(chunk.storage);
    self.end.set(chunk.end());
    chunks.push(chunk);
  }
}

#[derive(Debug)]
pub struct TypedArena<T> {
  start: Cell<*mut T>,
  end: Cell<*mut T>,

  chunks: RefCell<Vec<ArenaChunk<T>>>,
}
impl<T> TypedArena<T> {
  #[inline]
  pub const fn new() -> Self {
    Self {
      start: Cell::new(null_mut()),
      end: Cell::new(null_mut()),

      chunks: RefCell::new(Vec::new()),
    }
  }

  #[inline]
  pub fn alloc(&self, object: T) -> &mut T {
    if self.start == self.end {
      self.grow(1);
    };

    unsafe {
      if size_of::<T>() == 0 {
        self.start.set(self.start.get().wrapping_byte_add(1));
        let ptr = NonNull::<T>::dangling().as_ptr();
        write(ptr, object);
        return &mut *ptr;
      } else {
        let ptr = self.start.get();
        self.start.set(self.start.get().add(1));
        write(ptr, object);
        return &mut *ptr;
      };
    };
  }
  #[inline(never)]
  fn grow(&self, additional: usize) {
    let elem_size = cmp::max(1, size_of::<T>());
    let mut chunks = self.chunks.borrow_mut();
    let mut new_cap;
    if let Some(last_chunk) = chunks.last_mut() {
      if needs_drop::<T>() {
        let used_bytes = self.start.get().addr() - last_chunk.start().addr();
        last_chunk.entries = used_bytes / size_of::<T>();
      }

      new_cap = last_chunk.capacity.min(HUGE_PAGE / elem_size / 2);
      new_cap *= 2;
    } else {
      new_cap = PAGE / elem_size;
    }
    new_cap = cmp::max(additional, new_cap);

    let mut chunk = unsafe { ArenaChunk::<T>::new(new_cap) };
    self.start.set(chunk.start());
    self.end.set(chunk.end());
    chunks.push(chunk);
  }
}

#[derive(Debug)]
struct ArenaChunk<T = u8> {
  storage: *mut T,
  entries: usize,
  capacity: usize,
}
impl<T> ArenaChunk<T> {
  #[inline]
  pub unsafe fn new(capacity: usize) -> Self {
    debug_assert_ne!(size_of::<T>(), 0);
    debug_assert_ne!(capacity, 0);
    let mut vec = Vec::with_capacity(capacity);
    let storage = vec.as_mut_ptr();
    forget(vec);
    Self {
      storage,
      entries: 0,
      capacity,
    }
  }

  #[inline(always)]
  pub const fn start(&mut self) -> *mut T {
    self.storage
  }
  #[inline]
  pub const fn end(&mut self) -> *mut T {
    unsafe { return self.storage.add(self.capacity) };
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn dropless() {
    let arena = DroplessArena::new();

    arena.alloc(8);
    arena.alloc("object0");
    arena.alloc("object1");
    arena.alloc("object2");
    arena.alloc("object3");
    arena.alloc("object4");
    arena.alloc("object5");
    arena.alloc("object6");
    arena.alloc("object7");
    arena.alloc("object8");
    let o9 = arena.alloc("object9");

    dbg!(&arena);
    dbg!(&o9);
  }

  #[test]
  fn typed() {
    let arena = TypedArena::new();

    arena.alloc("object0".to_string());
    arena.alloc("object1".to_string());
    arena.alloc("object2".to_string());
    arena.alloc("object3".to_string());
    arena.alloc("object4".to_string());
    arena.alloc("object5".to_string());
    arena.alloc("object6".to_string());
    arena.alloc("object7".to_string());
    arena.alloc("object8".to_string());
    let o9 = arena.alloc("object9".to_string());

    dbg!(&arena);
    dbg!(&o9);
  }
}
