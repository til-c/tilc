use std::{
  alloc::Layout,
  cell::{Cell, RefCell},
  cmp,
  mem::{self, MaybeUninit},
  ptr::{self, NonNull},
};


const PAGE: usize = 4096;
const HUGE_PAGE: usize = 2 * 1024 * 1024;
#[inline(always)]
fn align_down(val: usize, align: usize) -> usize {
  debug_assert!(align.is_power_of_two());
  val & !(align - 1)
}

#[inline(always)]
fn align_up(val: usize, align: usize) -> usize {
  debug_assert!(align.is_power_of_two());
  (val + align - 1) & !(align - 1)
}

#[derive(Debug)]
pub struct TypedArena<T> {
  ptr: Cell<*mut T>,
  end: Cell<*mut T>,

  chunks: RefCell<Vec<ArenaChunk<T>>>,
}
impl<T> TypedArena<T> {
  pub fn new() -> Self {
    return Self {
      ptr: Cell::new(ptr::null_mut()),
      end: Cell::new(ptr::null_mut()),

      chunks: RefCell::default(),
    };
  }

  #[inline]
  pub fn alloc(&self, object: T) -> &mut T {
    if self.ptr == self.end {
      self.grow(1)
    }

    unsafe {
      if size_of::<T>() == 0 {
        self.ptr.set(self.ptr.get().wrapping_byte_add(1));
        let ptr = ptr::NonNull::<T>::dangling().as_ptr();
        // Don't drop the object. This `write` is equivalent to `forget`.
        ptr::write(ptr, object);
        return &mut *ptr;
      } else {
        let ptr = self.ptr.get();
        // Advance the pointer.
        self.ptr.set(self.ptr.get().add(1));
        // Write into uninitialized memory.
        ptr::write(ptr, object);
        return &mut *ptr;
      };
    };
  }
  #[inline(never)]
  fn grow(&self, additional: usize) {
    // We need the element size to convert chunk sizes (ranging from
    // PAGE to HUGE_PAGE bytes) to element counts.
    let elem_size = cmp::max(1, size_of::<T>());
    let mut chunks = self.chunks.borrow_mut();
    let mut new_cap;
    if let Some(last_chunk) = chunks.last_mut() {
      // If a type is `!needs_drop`, we don't need to keep track of how many elements
      // the chunk stores - the field will be ignored anyway.
      if mem::needs_drop::<T>() {
        // FIXME: this should *likely* use `offset_from`, but more
        // investigation is needed (including running tests in miri).
        let used_bytes = self.ptr.get().addr() - last_chunk.start().addr();
        last_chunk.entries = used_bytes / size_of::<T>();
      }

      // If the previous chunk's len is less than HUGE_PAGE
      // bytes, then this chunk will be least double the previous
      // chunk's size.
      new_cap = last_chunk.storage.len().min(HUGE_PAGE / elem_size / 2);
      new_cap *= 2;
    } else {
      new_cap = PAGE / elem_size;
    }
    // Also ensure that this chunk can fit `additional`.
    new_cap = cmp::max(additional, new_cap);

    let mut chunk = ArenaChunk::<T>::new(new_cap);
    self.ptr.set(chunk.start());
    self.end.set(chunk.end());
    chunks.push(chunk);
  }
}
impl<T> Default for TypedArena<T> {
  fn default() -> Self {
    return Self::new();
  }
}

#[derive(Debug, Default)]
pub struct DroplessArena {
  start: Cell<*mut u8>,
  end: Cell<*mut u8>,

  chunks: RefCell<Vec<ArenaChunk>>,
}
impl DroplessArena {
  const ALIGNMENT: usize = align_of::<usize>();


  pub fn new() -> Self {
    return Self {
      start: Cell::new(ptr::null_mut()),
      end: Cell::new(ptr::null_mut()),

      chunks: RefCell::default(),
    };
  }


  #[inline]
  pub fn alloc<T>(&self, object: T) -> &mut T {
    assert!(!mem::needs_drop::<T>());
    assert!(size_of::<T>() != 0);

    let mem = self.alloc_raw(Layout::new::<T>()) as *mut T;

    unsafe {
      // Write into uninitialized memory.
      ptr::write(mem, object);
      &mut *mem
    }
  }
  #[inline]
  pub fn alloc_raw(&self, layout: Layout) -> *mut u8 {
    assert!(layout.size() != 0);

    // This loop executes once or twice: if allocation fails the first
    // time, the `grow` ensures it will succeed the second time.
    loop {
      let start = self.start.get().addr();
      let old_end = self.end.get();
      let end = old_end.addr();

      // Align allocated bytes so that `self.end` stays aligned to
      // DROPLESS_ALIGNMENT.
      let bytes = align_up(layout.size(), Self::ALIGNMENT);

      assert!(end == align_down(end, Self::ALIGNMENT));

      if let Some(sub) = end.checked_sub(bytes) {
        let new_end = align_down(sub, layout.align());
        if start <= new_end {
          let new_end = old_end.with_addr(new_end);
          // `new_end` is aligned to DROPLESS_ALIGNMENT as `align_down`
          // preserves alignment as both `end` and `bytes` are already
          // aligned to DROPLESS_ALIGNMENT.
          self.end.set(new_end);
          return new_end;
        }
      }

      // No free space left. Allocate a new chunk to satisfy the request.
      // On failure the grow will panic or abort.
      self.grow(layout);
    }
  }
  #[inline(never)]
  fn grow(&self, layout: Layout) {
    let additional =
      layout.size() + cmp::max(Self::ALIGNMENT, layout.align()) - 1;

    let mut chunks = self.chunks.borrow_mut();
    let mut new_cap;
    if let Some(last_chunk) = chunks.last_mut() {
      new_cap = last_chunk.storage.len().min(HUGE_PAGE / 2);
      new_cap *= 2;
    } else {
      new_cap = PAGE;
    }
    // Also ensure that this chunk can fit `additional`.
    new_cap = cmp::max(additional, new_cap);

    let mut chunk = ArenaChunk::new(align_up(new_cap, PAGE));
    self.start.set(chunk.start());

    // Align the end to DROPLESS_ALIGNMENT.
    let end = align_down(chunk.end().addr(), Self::ALIGNMENT);

    // Make sure we don't go past `start`. This should not happen since the allocation
    // should be at least DROPLESS_ALIGNMENT - 1 bytes.
    debug_assert!(chunk.start().addr() <= end);

    self.end.set(chunk.end().with_addr(end));

    chunks.push(chunk);
  }
}


#[derive(Debug)]
pub struct ArenaChunk<T = u8> {
  storage: NonNull<[MaybeUninit<T>]>,
  entries: usize,
}
impl<T> ArenaChunk<T> {
  pub fn new(capacity: usize) -> Self {
    return Self {
      storage: NonNull::from(Box::leak(Box::new_uninit_slice(capacity))),
      entries: 0,
    };
  }

  #[inline(always)]
  pub fn start(&mut self) -> *mut T {
    return self.storage.as_ptr() as *mut T;
  }
  #[inline(always)]
  pub fn end(&mut self) -> *mut T {
    unsafe { return self.start().add(self.entries) };
  }
}


#[macro_export]
macro_rules! define_arenas {
  ($(
    $name:ident: $ty:ty,
  )*) => {
    pub trait ArenaAllocatable<'ctxt, C = tilc_arena::IsNotCopy>: Sized {
      fn allocate_on(self, arena: &'ctxt Arena) -> &'ctxt mut Self;
    }
    impl<'ctxt, T: Copy> ArenaAllocatable<'ctxt, tilc_arena::IsCopy> for T {
      fn allocate_on(self, arena: &'ctxt Arena) -> &'ctxt mut Self {
        return arena.dropless.alloc(self);
      }
    }
    $(
      impl<'ctxt> ArenaAllocatable<'ctxt, tilc_arena::IsNotCopy> for $ty {
        fn allocate_on(self, arena: &'ctxt Arena) -> &'ctxt mut Self {
          return arena.$name.alloc(self);
        }
      }
    )*

    #[derive(Debug, Default)]
    pub struct Arena {
      pub dropless: $crate::DroplessArena,
      $(pub $name: $crate::TypedArena<$ty>,)*
    }
    impl<'ctxt> Arena {
      pub fn alloc<T: ArenaAllocatable<'ctxt>>(&'ctxt self, value: T) -> &mut T {
        return value.allocate_on(self);
      }
    }
  };
}

pub struct IsCopy;
pub struct IsNotCopy;
