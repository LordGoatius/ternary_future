use core::ops::{BitXor, BitXorAssign};

use crate::Ternary;

#[inline]
pub const fn xor<const SIZE: usize>(
    _lhs: Ternary<SIZE>,
    _rhs: Ternary<SIZE>,
) -> Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    todo!()
}

impl<const SIZE: usize> BitXor<Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;

    fn bitxor(self, rhs: Ternary<SIZE>) -> Self::Output {
        xor(self, rhs)
    }
}

impl<const SIZE: usize> BitXor<Ternary<SIZE>> for &Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn bitxor(self, rhs: Ternary<SIZE>) -> Self::Output {
        xor(*self, rhs)
    }
}

impl<const SIZE: usize> BitXor<Ternary<SIZE>> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn bitxor(self, rhs: Ternary<SIZE>) -> Self::Output {
        xor(*self, rhs)
    }
}

impl<const SIZE: usize> BitXor<&Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn bitxor(self, rhs: &Ternary<SIZE>) -> Self::Output {
        xor(self, *rhs)
    }
}

impl<const SIZE: usize> BitXor<&Ternary<SIZE>> for &Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn bitxor(self, rhs: &Ternary<SIZE>) -> Self::Output {
        xor(*self, *rhs)
    }
}

impl<const SIZE: usize> BitXor<&Ternary<SIZE>> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn bitxor(self, rhs: &Ternary<SIZE>) -> Self::Output {
        xor(*self, *rhs)
    }
}

impl<const SIZE: usize> BitXor<&mut Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn bitxor(self, rhs: &mut Ternary<SIZE>) -> Self::Output {
        xor(self, *rhs)
    }
}

impl<const SIZE: usize> BitXor<&mut Ternary<SIZE>> for &Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn bitxor(self, rhs: &mut Ternary<SIZE>) -> Self::Output {
        xor(*self, *rhs)
    }
}

impl<const SIZE: usize> BitXor<&mut Ternary<SIZE>> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn bitxor(self, rhs: &mut Ternary<SIZE>) -> Self::Output {
        xor(*self, *rhs)
    }
}

impl<const SIZE: usize> BitXorAssign<Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn bitxor_assign(&mut self, rhs: Ternary<SIZE>) {
        *self = *self & rhs;
    }
}

impl<const SIZE: usize> BitXorAssign<&Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn bitxor_assign(&mut self, rhs: &Ternary<SIZE>) {
        *self = *self & *rhs;
    }
}

impl<const SIZE: usize> BitXorAssign<&mut Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn bitxor_assign(&mut self, rhs: &mut Ternary<SIZE>) {
        *self = *self & *rhs;
    }
}

impl<const SIZE: usize> BitXorAssign<&mut Ternary<SIZE>> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn bitxor_assign(&mut self, rhs: &mut Ternary<SIZE>) {
        **self = **self & *rhs;
    }
}
