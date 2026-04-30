use crate::Ternary;

pub mod shl;
pub mod shr;

const fn shl<const SIZE: usize>(lhs: Ternary<SIZE>, rhs: isize) -> Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    let Ternary { pos, neg } = lhs;
    let pos = pos << rhs;
    let neg = neg << rhs;
    Ternary { pos, neg }
}

const fn shr<const SIZE: usize>(lhs: Ternary<SIZE>, rhs: isize) -> Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    let Ternary { pos, neg } = lhs;
    let pos = pos >> rhs;
    let neg = neg >> rhs;
    Ternary { pos, neg }
}
