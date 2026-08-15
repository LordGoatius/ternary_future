use core::ops::{Shl, ShlAssign};

use super::shl;
use crate::Ternary;

const impl<const SIZE: usize> Shl<isize> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn shl(self, rhs: isize) -> Self::Output {
        shl(self, rhs)
    }
}

const impl<const SIZE: usize> Shl<isize> for &Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn shl(self, rhs: isize) -> Self::Output {
        shl(*self, rhs)
    }
}

const impl<const SIZE: usize> Shl<isize> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn shl(self, rhs: isize) -> Self::Output {
        shl(*self, rhs)
    }
}

const impl<const SIZE: usize> Shl<&isize> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn shl(self, rhs: &isize) -> Self::Output {
        shl(self, *rhs)
    }
}

const impl<const SIZE: usize> Shl<&isize> for &Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn shl(self, rhs: &isize) -> Self::Output {
        shl(*self, *rhs)
    }
}

const impl<const SIZE: usize> Shl<&isize> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn shl(self, rhs: &isize) -> Self::Output {
        shl(*self, *rhs)
    }
}

const impl<const SIZE: usize> Shl<&mut isize> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn shl(self, rhs: &mut isize) -> Self::Output {
        shl(self, *rhs)
    }
}

const impl<const SIZE: usize> Shl<&mut isize> for &Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn shl(self, rhs: &mut isize) -> Self::Output {
        shl(*self, *rhs)
    }
}

const impl<const SIZE: usize> Shl<&mut isize> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn shl(self, rhs: &mut isize) -> Self::Output {
        shl(*self, *rhs)
    }
}

const impl<const SIZE: usize> ShlAssign<isize> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn shl_assign(&mut self, rhs: isize) {
        *self = *self << rhs;
    }
}

const impl<const SIZE: usize> ShlAssign<&isize> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn shl_assign(&mut self, rhs: &isize) {
        *self = *self << *rhs;
    }
}

const impl<const SIZE: usize> ShlAssign<&mut isize> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn shl_assign(&mut self, rhs: &mut isize) {
        *self = *self << *rhs;
    }
}

const impl<const SIZE: usize> ShlAssign<&mut isize> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn shl_assign(&mut self, rhs: &mut isize) {
        **self = **self << *rhs;
    }
}
