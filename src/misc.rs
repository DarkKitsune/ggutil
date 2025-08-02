use std::num::{NonZeroIsize, NonZeroUsize};

/// Implemented by ``ConstEval<true>``.
pub trait IsTrue {}
/// Implemented by ``ConstEval<false>``.
pub trait IsFalse {}

/// Implements ``IsTrue`` for ``ConstEval<true>``, and ``IsFalse`` for ``ConstEval<false>``.
/// Useful for conditional compilation based on const generics.
pub struct ConstEval<const CONDITION: bool>;

impl IsTrue for ConstEval<true> {}
impl IsFalse for ConstEval<false> {}

/// Returns a non-zero value at compile-time. This function is not implemented for zero.
pub const fn nonzero_usize<const N: usize>() -> NonZeroUsize
where
    ConstEval<{ N != 0 }>: IsTrue,
{
    unsafe { NonZeroUsize::new_unchecked(N) }
}

/// Returns a non-zero value at compile-time. This function is not implemented for zero.
pub const fn nonzero_isize<const N: isize>() -> NonZeroIsize
where
    ConstEval<{ N != 0 }>: IsTrue,
{
    unsafe { NonZeroIsize::new_unchecked(N) }
}