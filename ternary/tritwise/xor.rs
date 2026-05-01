use core::ops::{BitXor, BitXorAssign};

use crate::{
    helper::neg,
    tritwise::{and::and, or::or},
    Ternary,
};

/// Truth Table
/// A xor B
///            B
///        −1  0  +1
///       +-----------
///    −1 | −1  0  +1
/// A   0 |  0  0  0
///    +1 | +1  0  −1
/// 
#[inline]
pub const fn xor<const SIZE: usize>(
    lhs: Ternary<SIZE>,
    rhs: Ternary<SIZE>,
) -> Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    // TODO: Replace with better and faster impl.
    // Don't know how much the compiler can optimize this
    and(or(lhs, rhs), neg(and(lhs, rhs)))
}

impl<const SIZE: usize> const BitXor<Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;

    fn bitxor(self, rhs: Ternary<SIZE>) -> Self::Output {
        xor(self, rhs)
    }
}

impl<const SIZE: usize> const BitXor<Ternary<SIZE>> for &Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn bitxor(self, rhs: Ternary<SIZE>) -> Self::Output {
        xor(*self, rhs)
    }
}

impl<const SIZE: usize> const BitXor<Ternary<SIZE>> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn bitxor(self, rhs: Ternary<SIZE>) -> Self::Output {
        xor(*self, rhs)
    }
}

impl<const SIZE: usize> const BitXor<&Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn bitxor(self, rhs: &Ternary<SIZE>) -> Self::Output {
        xor(self, *rhs)
    }
}

impl<const SIZE: usize> const BitXor<&Ternary<SIZE>> for &Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn bitxor(self, rhs: &Ternary<SIZE>) -> Self::Output {
        xor(*self, *rhs)
    }
}

impl<const SIZE: usize> const BitXor<&Ternary<SIZE>> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn bitxor(self, rhs: &Ternary<SIZE>) -> Self::Output {
        xor(*self, *rhs)
    }
}

impl<const SIZE: usize> const BitXor<&mut Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn bitxor(self, rhs: &mut Ternary<SIZE>) -> Self::Output {
        xor(self, *rhs)
    }
}

impl<const SIZE: usize> const BitXor<&mut Ternary<SIZE>> for &Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn bitxor(self, rhs: &mut Ternary<SIZE>) -> Self::Output {
        xor(*self, *rhs)
    }
}

impl<const SIZE: usize> const BitXor<&mut Ternary<SIZE>> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn bitxor(self, rhs: &mut Ternary<SIZE>) -> Self::Output {
        xor(*self, *rhs)
    }
}

impl<const SIZE: usize> const BitXorAssign<Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn bitxor_assign(&mut self, rhs: Ternary<SIZE>) {
        *self = *self ^ rhs;
    }
}

impl<const SIZE: usize> const BitXorAssign<&Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn bitxor_assign(&mut self, rhs: &Ternary<SIZE>) {
        *self ^= *rhs;
    }
}

impl<const SIZE: usize> const BitXorAssign<&mut Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn bitxor_assign(&mut self, rhs: &mut Ternary<SIZE>) {
        *self ^= *rhs;
    }
}

impl<const SIZE: usize> const BitXorAssign<&mut Ternary<SIZE>> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn bitxor_assign(&mut self, rhs: &mut Ternary<SIZE>) {
        **self ^= *rhs;
    }
}
