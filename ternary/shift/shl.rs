use core::ops::{Shl, ShlAssign};

use super::shl;
use crate::Ternary;

impl<const SIZE: usize> const Shl<isize> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn shl(self, rhs: isize) -> Self::Output {
        shl(self, rhs)
    }
}

impl<const SIZE: usize> const Shl<isize> for &Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn shl(self, rhs: isize) -> Self::Output {
        shl(*self, rhs)
    }
}

impl<const SIZE: usize> const Shl<isize> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn shl(self, rhs: isize) -> Self::Output {
        shl(*self, rhs)
    }
}

impl<const SIZE: usize> const Shl<&isize> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn shl(self, rhs: &isize) -> Self::Output {
        shl(self, *rhs)
    }
}

impl<const SIZE: usize> const Shl<&isize> for &Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn shl(self, rhs: &isize) -> Self::Output {
        shl(*self, *rhs)
    }
}

impl<const SIZE: usize> const Shl<&isize> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn shl(self, rhs: &isize) -> Self::Output {
        shl(*self, *rhs)
    }
}

impl<const SIZE: usize> const Shl<&mut isize> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn shl(self, rhs: &mut isize) -> Self::Output {
        shl(self, *rhs)
    }
}

impl<const SIZE: usize> const Shl<&mut isize> for &Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn shl(self, rhs: &mut isize) -> Self::Output {
        shl(*self, *rhs)
    }
}

impl<const SIZE: usize> const Shl<&mut isize> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn shl(self, rhs: &mut isize) -> Self::Output {
        shl(*self, *rhs)
    }
}

impl<const SIZE: usize> const ShlAssign<isize> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn shl_assign(&mut self, rhs: isize) {
        *self = *self << rhs;
    }
}

impl<const SIZE: usize> const ShlAssign<&isize> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn shl_assign(&mut self, rhs: &isize) {
        *self = *self << *rhs;
    }
}

impl<const SIZE: usize> const ShlAssign<&mut isize> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn shl_assign(&mut self, rhs: &mut isize) {
        *self = *self << *rhs;
    }
}

impl<const SIZE: usize> const ShlAssign<&mut isize> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn shl_assign(&mut self, rhs: &mut isize) {
        **self = **self << *rhs;
    }
}
