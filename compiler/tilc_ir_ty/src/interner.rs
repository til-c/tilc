use core::fmt;

pub trait Interner {
  type ErrorGuaranteed: fmt::Debug;
}
