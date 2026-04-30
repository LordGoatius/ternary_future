use core::ops::{Shr, ShrAssign};

use super::shr;
use crate::Ternary;

impl<const SIZE: usize> const Shr<isize> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn shr(self, rhs: isize) -> Self::Output {
        shr(self, rhs)
    }
}

impl<const SIZE: usize> const Shr<isize> for &Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn shr(self, rhs: isize) -> Self::Output {
        shr(*self, rhs)
    }
}

impl<const SIZE: usize> const Shr<isize> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn shr(self, rhs: isize) -> Self::Output {
        shr(*self, rhs)
    }
}

impl<const SIZE: usize> const Shr<&isize> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn shr(self, rhs: &isize) -> Self::Output {
        shr(self, *rhs)
    }
}

impl<const SIZE: usize> const Shr<&isize> for &Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn shr(self, rhs: &isize) -> Self::Output {
        shr(*self, *rhs)
    }
}

impl<const SIZE: usize> const Shr<&isize> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn shr(self, rhs: &isize) -> Self::Output {
        shr(*self, *rhs)
    }
}

impl<const SIZE: usize> const Shr<&mut isize> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn shr(self, rhs: &mut isize) -> Self::Output {
        shr(self, *rhs)
    }
}

impl<const SIZE: usize> const Shr<&mut isize> for &Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn shr(self, rhs: &mut isize) -> Self::Output {
        shr(*self, *rhs)
    }
}

impl<const SIZE: usize> const Shr<&mut isize> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn shr(self, rhs: &mut isize) -> Self::Output {
        shr(*self, *rhs)
    }
}

impl<const SIZE: usize> const ShrAssign<isize> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn shr_assign(&mut self, rhs: isize) {
        *self = *self >> rhs;
    }
}

impl<const SIZE: usize> const ShrAssign<&isize> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn shr_assign(&mut self, rhs: &isize) {
        *self = *self >> *rhs;
    }
}

impl<const SIZE: usize> const ShrAssign<&mut isize> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn shr_assign(&mut self, rhs: &mut isize) {
        *self = *self >> *rhs;
    }
}

impl<const SIZE: usize> const ShrAssign<&mut isize> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn shr_assign(&mut self, rhs: &mut isize) {
        **self = **self >> *rhs;
    }
}
