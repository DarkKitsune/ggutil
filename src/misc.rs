pub trait IsTrue {}
pub trait IsFalse {}

/// Implements ``IsTrue`` for ``ConstEval<true>``, and ``IsFalse`` for ``ConstEval<false>``.
/// Useful for conditional compilation based on const generics.
pub struct ConstEval<const CONDITION: bool>;

impl IsTrue for ConstEval<true> {}
impl IsFalse for ConstEval<false> {}
