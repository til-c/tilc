use std::sync::Arc;

use tilc_ast::Sandyq;
use tilc_data_structures::Holder;

use crate::ResolverAstLowering;

macro_rules! define_arenas {
  ($(
    $name:ident: $ty:ty,
  )*) => {
    #[derive(Debug)]
    pub struct Arena {
      dropless: ::tilc_data_structures::DroplessArena,
      $($name: ::tilc_data_structures::TypedArena<$ty>),*
    }
    impl<'ctxt> Arena {
      pub const fn new() -> Self {
        Self {
          dropless: ::tilc_data_structures::DroplessArena::new(),
          $($name: ::tilc_data_structures::TypedArena::<$ty>::new()),*
        }
      }

      pub fn alloc<T: ArenaAllocatable<'ctxt>>(&'ctxt self, value: T) -> &'ctxt mut T {
        value.allocate_on(self)
      }
    }

    pub trait ArenaAllocatable<'ctxt, C = $crate::IsNotCopy>: Sized {
      fn allocate_on(self, arena: &'ctxt Arena) -> &'ctxt mut Self;
    }
    impl<'ctxt, T: Copy> ArenaAllocatable<'ctxt, $crate::IsCopy> for T {
      fn allocate_on(self, arena: &'ctxt Arena) -> &'ctxt mut Self {
        arena.dropless.alloc(self)
      }
    }
    $(impl<'ctxt> ArenaAllocatable<'ctxt, $crate::IsNotCopy> for $ty {
      fn allocate_on(self, arena: &'ctxt Arena) -> &'ctxt mut Self {
        arena.$name.alloc(self)
      }
    })*
  };
}

define_arenas! {
  resolver_for_lowering: Holder<(ResolverAstLowering, Arc<Sandyq>)>,
  sandyq_for_resolving: Holder<Sandyq>,
}

pub struct IsNotCopy;
pub struct IsCopy;
