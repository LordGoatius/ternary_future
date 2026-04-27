use crate::tern_next::Ternary;

#[inline]
pub fn xor<const S1: usize, const S2: usize>(lhs: Ternary<S1>, rhs: Ternary<S2>) -> Ternary<S1>
    where
        [(); S1 + (usize::MAX - 32)]:,
        [(); S2 + (usize::MAX - 32)]:,
        [(); S1 - S2]:,
{
    todo!()
}
