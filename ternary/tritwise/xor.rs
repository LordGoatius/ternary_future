use core::ops::{BitXor, BitXorAssign};

use crate::Ternary;

#[inline]
pub const fn xor<const S1: usize, const S2: usize>(
    _lhs: Ternary<S1>,
    _rhs: Ternary<S2>,
) -> Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    todo!()
}

impl<const S1: usize, const S2: usize> BitXor<Ternary<S2>> for Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;

    fn bitxor(self, rhs: Ternary<S2>) -> Self::Output {
        xor(self, rhs)
    }
}

impl<const S1: usize, const S2: usize> BitXor<Ternary<S2>> for &Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn bitxor(self, rhs: Ternary<S2>) -> Self::Output {
        xor(*self, rhs)
    }
}

impl<const S1: usize, const S2: usize> BitXor<Ternary<S2>> for &mut Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn bitxor(self, rhs: Ternary<S2>) -> Self::Output {
        xor(*self, rhs)
    }
}

impl<const S1: usize, const S2: usize> BitXor<&Ternary<S2>> for Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn bitxor(self, rhs: &Ternary<S2>) -> Self::Output {
        xor(self, *rhs)
    }
}

impl<const S1: usize, const S2: usize> BitXor<&Ternary<S2>> for &Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn bitxor(self, rhs: &Ternary<S2>) -> Self::Output {
        xor(*self, *rhs)
    }
}

impl<const S1: usize, const S2: usize> BitXor<&Ternary<S2>> for &mut Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn bitxor(self, rhs: &Ternary<S2>) -> Self::Output {
        xor(*self, *rhs)
    }
}

impl<const S1: usize, const S2: usize> BitXor<&mut Ternary<S2>> for Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn bitxor(self, rhs: &mut Ternary<S2>) -> Self::Output {
        xor(self, *rhs)
    }
}

impl<const S1: usize, const S2: usize> BitXor<&mut Ternary<S2>> for &Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn bitxor(self, rhs: &mut Ternary<S2>) -> Self::Output {
        xor(*self, *rhs)
    }
}

impl<const S1: usize, const S2: usize> BitXor<&mut Ternary<S2>> for &mut Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn bitxor(self, rhs: &mut Ternary<S2>) -> Self::Output {
        xor(*self, *rhs)
    }
}

impl<const S1: usize, const S2: usize> BitXorAssign<Ternary<S2>> for Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    fn bitxor_assign(&mut self, rhs: Ternary<S2>) {
        *self = *self & rhs;
    }
}

impl<const S1: usize, const S2: usize> BitXorAssign<&Ternary<S2>> for Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    fn bitxor_assign(&mut self, rhs: &Ternary<S2>) {
        *self = *self & *rhs;
    }
}

impl<const S1: usize, const S2: usize> BitXorAssign<&mut Ternary<S2>> for Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    fn bitxor_assign(&mut self, rhs: &mut Ternary<S2>) {
        *self = *self & *rhs;
    }
}

impl<const S1: usize, const S2: usize> BitXorAssign<&mut Ternary<S2>> for &mut Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    fn bitxor_assign(&mut self, rhs: &mut Ternary<S2>) {
        **self = **self & *rhs;
    }
}
