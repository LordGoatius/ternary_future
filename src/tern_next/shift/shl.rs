use std::ops::{Shl, ShlAssign};

use super::shl;
use crate::tern_next::Ternary;

impl<const S1: usize> Shl<isize> for Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
{
    type Output = Ternary<S1>;
    fn shl(self, rhs: isize) -> Self::Output {
        shl(self, rhs)
    }
}

impl<const S1: usize> Shl<isize> for &Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
{
    type Output = Ternary<S1>;
    fn shl(self, rhs: isize) -> Self::Output {
        shl(*self, rhs)
    }
}

impl<const S1: usize> Shl<isize> for &mut Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
{
    type Output = Ternary<S1>;
    fn shl(self, rhs: isize) -> Self::Output {
        shl(*self, rhs)
    }
}

impl<const S1: usize> Shl<&isize> for Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
{
    type Output = Ternary<S1>;
    fn shl(self, rhs: &isize) -> Self::Output {
        shl(self, *rhs)
    }
}

impl<const S1: usize> Shl<&isize> for &Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
{
    type Output = Ternary<S1>;
    fn shl(self, rhs: &isize) -> Self::Output {
        shl(*self, *rhs)
    }
}

impl<const S1: usize> Shl<&isize> for &mut Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
{
    type Output = Ternary<S1>;
    fn shl(self, rhs: &isize) -> Self::Output {
        shl(*self, *rhs)
    }
}

impl<const S1: usize> Shl<&mut isize> for Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
{
    type Output = Ternary<S1>;
    fn shl(self, rhs: &mut isize) -> Self::Output {
        shl(self, *rhs)
    }
}

impl<const S1: usize> Shl<&mut isize> for &Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
{
    type Output = Ternary<S1>;
    fn shl(self, rhs: &mut isize) -> Self::Output {
        shl(*self, *rhs)
    }
}

impl<const S1: usize> Shl<&mut isize> for &mut Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
{
    type Output = Ternary<S1>;
    fn shl(self, rhs: &mut isize) -> Self::Output {
        shl(*self, *rhs)
    }
}

impl<const SIZE: usize> ShlAssign<isize> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn shl_assign(&mut self, rhs: isize) {
        *self = *self >> rhs;
    }
}

impl<const S1: usize> ShlAssign<&isize> for Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
{
    fn shl_assign(&mut self, rhs: &isize) {
        *self = *self >> *rhs;
    }
}

impl<const S1: usize> ShlAssign<&mut isize> for Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
{
    fn shl_assign(&mut self, rhs: &mut isize) {
        *self = *self >> *rhs;
    }
}

impl<const S1: usize> ShlAssign<&mut isize> for &mut Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
{
    fn shl_assign(&mut self, rhs: &mut isize) {
        **self = **self >> *rhs;
    }
}
