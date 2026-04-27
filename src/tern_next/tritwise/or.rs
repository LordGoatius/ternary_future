use crate::tern_next::Ternary;

// binary and ternary `or` functions like a `max` function on each bit/trit.
#[inline]
pub fn or<const S1: usize, const S2: usize>(lhs: Ternary<S1>, rhs: Ternary<S2>) -> Ternary<S1>
    where
        [(); S1 + (usize::MAX - 32)]:,
        [(); S2 + (usize::MAX - 32)]:,
        [(); S1 - S2]:,
{
    todo!()
}
