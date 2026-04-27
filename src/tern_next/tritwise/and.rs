use crate::tern_next::Ternary;

// binary and ternary `and` functions like a `min` function on each bit/trit.
#[inline]
pub fn and<const S1: usize, const S2: usize>(lhs: Ternary<S1>, rhs: Ternary<S2>) -> Ternary<S1>
    where
        [(); S1 + (usize::MAX - 32)]:,
        [(); S2 + (usize::MAX - 32)]:,
        [(); S1 - S2]:,
{
    let Ternary { pos, neg } = lhs;
    let Ternary { pos: pos1, neg: neg1 } = rhs;

    todo!()
}
